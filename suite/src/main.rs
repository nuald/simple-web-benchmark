#[macro_use]
extern crate clap;

use chrono::prelude::*;

use clap::{App, Arg};

use itertools::Itertools;
use plotters::data::fitting_range;
use plotters::prelude::*;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};

mod errors;

type UnitResult = Result<(), Box<dyn Error>>;
type UnsignedResult = Result<u32, Box<dyn Error>>;

struct Cmd<'a> {
    title: &'a str,
    build: Box<dyn Fn() -> UnitResult>,
    run: Box<dyn Fn() -> UnsignedResult>,
}

thread_local! {
    static LSOF_PATTERN: Regex = Regex::new(r"p(\d+)").unwrap();
    static CSV_PATTERN: Regex = Regex::new(
        r"(?x)
(?P<responseTime>[\d\.]+),
(?P<dnsLookup>[\d\.]+),
(?P<dns>[\d\.]+),
(?P<requestWrite>[\d\.]+),
(?P<responseDelay>[\d\.]+),
(?P<responseRead>[\d\.]+),
(?P<statusCode>\d+),
(?P<offset>[\d\.]+)").unwrap();
}

const ATTEMPTS: u32 = 30;

fn log(msg: &str) {
    let local = Local::now();
    println!("{} {}", local.format("%H:%M:%S"), msg)
}

fn exec(cmd: &mut Command) -> UnitResult {
    let mut child = cmd.spawn()?;
    let status = child.wait()?;
    if !status.success() {
        Err(Box::new(errors::ProcessError::new(status.code())))
    } else {
        Ok(())
    }
}

fn pexec(cmd: &mut Command) -> UnitResult {
    log(&format!("{:?}", cmd));
    exec(cmd)
}

fn pspawn(cmd: &mut Command) -> UnsignedResult {
    log(&format!("{:?}", cmd));
    fs::remove_file(".pid")?;
    cmd.spawn()?;
    print!("Waiting");
    for _ in 0..ATTEMPTS {
        if let Ok(content) = fs::read_to_string(".pid") {
            return Ok(content.parse().unwrap());
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        print!(".")
    }
    Err(Box::new(errors::PidError {}))
}

fn kill(pid: u32) {
    // Ignore any errors as the process could be finished already
    let _ = exec(Command::new("kill").args(&["-9", &format!("-{}", pid)]));
    let _ = exec(Command::new("kill").args(&["-9", &pid.to_string()]));
}

fn kill_processes() -> Result<bool, Box<dyn Error>> {
    let output = Command::new("lsof")
        .args(&["-Fp", "-i", ":3000"])
        .output()?;
    let output = String::from_utf8(output.stdout)?;
    let found = LSOF_PATTERN.with(|re| {
        let mut result = false;
        if let Some(captures) = re.captures(&output) {
            if let Some(m) = captures.get(1) {
                kill(m.as_str().parse().unwrap());
                result = true;
            }
        }
        result
    });
    Ok(found)
}

fn run_hey(additional: &[&str], capture: bool) -> Result<Option<String>, Box<dyn Error>> {
    let stdout = if capture {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };
    let mut cmd = Command::new("hey");
    cmd.stdout(stdout)
        .args(&["-n", "50000", "-c", "256", "-t", "10"])
        .args(additional);
    if capture {
        let child = cmd.spawn()?;
        let output = child.wait_with_output()?;
        let status = output.status;
        if !status.success() {
            Err(Box::new(errors::ProcessError::new(status.code())))
        } else {
            let content = String::from_utf8(output.stdout)?;
            Ok(Some(content))
        }
    } else {
        exec(&mut cmd)?;
        Ok(None)
    }
}

fn run_benchmark(lang: &str, is_index: bool) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut url = String::from("http://127.0.0.1:3000/");
    if !is_index {
        url.push_str("greeting/hello");
    }

    // First run, for JIT
    run_hey(&["-o", "csv", &url], true)?;

    // Second run, for UI
    println!("[{}] {}", lang, url);
    run_hey(&[&url], false)?;

    // Third run, for stats
    if let Some(content) = run_hey(&["-o", "csv", &url], true).unwrap() {
        CSV_PATTERN.with(|re| {
            let values = content
                .split('\n')
                .map(|line| {
                    let mut result = None;
                    if let Some(captures) = re.captures(line) {
                        if let Some(m) = captures.name("responseTime") {
                            let double_value: f64 = m.as_str().parse().unwrap();
                            result = Some(double_value * 1000.0)
                        }
                    }
                    result
                })
                .filter_map(|x| x)
                .collect();
            Ok(values)
        })
    } else {
        Ok(vec![])
    }
}

fn run(lang_cmd: &Cmd, verbose: bool) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    (lang_cmd.build)()?;
    let pid = (lang_cmd.run)()?;
    let index_values = run_benchmark(lang_cmd.title, true)?;
    let pattern_values = run_benchmark(lang_cmd.title, false)?;
    if verbose {
        log(&format!("Killing {} process tree...", pid));
    }
    kill(pid);
    Ok((index_values, pattern_values))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lang_cmds = HashMap::new();
    lang_cmds.insert(
        "go",
        Cmd {
            title: "Go",
            build: Box::new(|| {
                pexec(Command::new("go").args(&["build", "-o", "go/build/main", "go/main.go"]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("go/build/main"))),
        },
    );
    lang_cmds.insert(
        "rust_hyper",
        Cmd {
            title: "Rust/hyper",
            build: Box::new(|| {
                pexec(Command::new("cargo").args(&[
                    "build",
                    "--manifest-path",
                    "rust/hyper/Cargo.toml",
                    "--release",
                ]))
            }),
            run: Box::new(|| {
                pspawn(&mut Command::new(
                    "rust/hyper/target/release/simple-web-server",
                ))
            }),
        },
    );
    lang_cmds.insert(
        "scala",
        Cmd {
            title: "Scala/Akka",
            build: Box::new(|| {
                pexec(Command::new("make").args(&["-C", "scala", "clean", "target/library.jar"]))
            }),
            run: Box::new(|| {
                let cp = format!(
                    "scala/target/library.jar:{}",
                    fs::read_to_string("scala/target/classpath.line").unwrap()
                );
                pspawn(&mut Command::new("scala").args(&["-cp", &cp, "lite.WebServer"]))
            }),
        },
    );
    lang_cmds.insert(
        "java",
        Cmd {
            title: "Java/Spring Boot",
            build: Box::new(|| {
                pexec(Command::new("make").args(&["-C", "java", "clean", "target/library.jar"]))
            }),
            run: Box::new(|| {
                let cp = format!(
                    "java/target/library.jar:{}",
                    fs::read_to_string("java/target/classpath.line").unwrap()
                );
                pspawn(&mut Command::new("java").args(&[
                    "-cp",
                    &cp,
                    "-Dserver.port=3000",
                    "hello.SampleController",
                ]))
            }),
        },
    );
    lang_cmds.insert(
        "nodejs",
        Cmd {
            title: "Node.js",
            build: Box::new(|| Ok(())),
            run: Box::new(|| pspawn(&mut Command::new("node").args(&["nodejs/main.js"]))),
        },
    );
    lang_cmds.insert(
        "ldc2",
        Cmd {
            title: "D (LDC/vibe.d)",
            build: Box::new(|| {
                pexec(Command::new("dub").args(&[
                    "build",
                    "--root=d",
                    "--compiler=ldc2",
                    "--build=release",
                    "--config=ldc",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("d/build/ldc/vibedtest"))),
        },
    );
    lang_cmds.insert(
        "dmd",
        Cmd {
            title: "D (DMD/vibe.d)",
            build: Box::new(|| {
                pexec(Command::new("dub").args(&[
                    "build",
                    "--root=d",
                    "--compiler=dmd",
                    "--build=release",
                    "--config=dmd",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("d/build/dmd/vibedtest"))),
        },
    );
    lang_cmds.insert(
        "crystal",
        Cmd {
            title: "Crystal",
            build: Box::new(|| {
                pexec(Command::new("crystal").args(&[
                    "build",
                    "--release",
                    "--no-debug",
                    "-o",
                    "crystal/server",
                    "crystal/server.cr",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("crystal/server"))),
        },
    );
    lang_cmds.insert(
        "rust_rocket",
        Cmd {
            title: "Rust/rocket",
            build: Box::new(|| {
                pexec(Command::new("cargo").args(&[
                    "build",
                    "--manifest-path",
                    "rust/rocket/Cargo.toml",
                    "--release",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("rust/rocket/target/release/rust-rocket"))),
        },
    );
    lang_cmds.insert(
        "python",
        Cmd {
            title: "PyPy3/Twisted",
            build: Box::new(|| Ok(())),
            run: Box::new(|| pspawn(&mut Command::new("pypy3").args(&["python/twist.py"]))),
        },
    );
    lang_cmds.insert(
        "php",
        Cmd {
            title: "PHP/Swoole",
            build: Box::new(|| Ok(())),
            run: Box::new(|| {
                pspawn(&mut Command::new("php").args(&[
                    "-c",
                    "php/swoole/php.ini",
                    "php/swoole/main.php",
                ]))
            }),
        },
    );

    let default_img = "result.svg";
    let matches = App::new("Simple Web Benchmark runner")
        .version(crate_version!())
        .usage("cargo run --manifest-path suite/Cargo.toml -- [FLAGS] [OPTIONS] <lang>...")
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("file")
                .help(&format!(
                    "Sets an image file to generate ({} by default)",
                    default_img
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .help("Enables the verbose output"),
        )
        .arg(
            Arg::with_name("lang")
                .index(1)
                .multiple(true)
                .required(true)
                .help("Sets the languages to test ('all' for all)"),
        )
        .after_help(
            &format!(
                "The following languages are supported: {}.",
                lang_cmds.keys().join(", ")
            )[..],
        )
        .get_matches();
    let verbose = matches.is_present("verbose");
    let mut langs: Vec<&str> = matches.values_of("lang").unwrap().collect();
    if langs.iter().any(|&x| x == "all") {
        langs = lang_cmds.iter().map(|(key, _)| *key).collect();
    }
    let img = matches.value_of("out").unwrap_or(default_img);

    let mut dataset = Vec::new();
    for (lang, lang_cmd) in &lang_cmds {
        if langs.iter().position(|x| x == lang).is_none() {
            continue;
        }

        while kill_processes().unwrap() {
            // kill until all died
        }
        let (index_values, pattern_values) = run(lang_cmd, verbose).unwrap();
        dataset.push((
            String::from(lang_cmd.title),
            "Index URL Request",
            Quartiles::new(&index_values),
        ));
        dataset.push((
            String::from(lang_cmd.title),
            "Pattern URL Request",
            Quartiles::new(&pattern_values),
        ));
    }

    let category = Category::new(
        "Language",
        dataset
            .iter()
            .unique_by(|x| x.0.clone())
            .sorted_by(|a, b| a.2.median().partial_cmp(&b.2.median()).unwrap())
            .map(|x| x.0.clone())
            .collect(),
    );

    let mut colors = [RED, BLUE].iter();
    let mut offsets = (-15..).step_by(30);
    let mut series = HashMap::new();
    for x in dataset.iter() {
        let entry = series
            .entry(x.1)
            .or_insert_with(|| (Vec::new(), colors.next().unwrap(), offsets.next().unwrap()));
        entry.0.push((x.0.clone(), &x.2));
    }

    let values: Vec<f32> = dataset
        .iter()
        .map(|x| x.2.values().to_vec())
        .flatten()
        .collect();
    let values_range = fitting_range(values.iter());

    let root = SVGBackend::new(img, (480, 640)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(0.0..values_range.end + 1.0, category.range())?;

    chart
        .configure_mesh()
        .x_desc("Response, ms")
        .y_desc(category.name())
        .line_style_2(&WHITE)
        .draw()?;

    for (label, (values, style, offset)) in &series {
        let style_copy = *style;
        chart
            .draw_series(values.iter().map(|x| {
                Boxplot::new_horizontal(category.get(&x.0).unwrap(), &x.1)
                    .width(20)
                    .whisker_width(0.5)
                    .style(*style)
                    .offset(*offset)
            }))?
            .label(*label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style_copy));
    }
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
