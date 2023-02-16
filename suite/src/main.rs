#[macro_use]
extern crate clap;

use chrono::prelude::*;

use clap::Arg;

use itertools::Itertools;
use plotters::data::fitting_range;
use plotters::prelude::*;

use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

mod errors;

type UnitResult = Result<(), Box<dyn Error>>;
type UnsignedResult = Result<u32, Box<dyn Error>>;

const INDEX: &str = "Index URL Request";
const PATTERN: &str = "Pattern URL Request";

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
    log(&format!("{cmd:?}"));
    exec(cmd)
}

fn pspawn(cmd: &mut Command) -> UnsignedResult {
    log(&format!("{cmd:?}"));
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
    let _ = exec(Command::new("kill").args([&pid.to_string()]));
}

fn kill_processes() -> Result<bool, Box<dyn Error>> {
    let output = Command::new("lsof").args(["-Fp", "-i", ":3000"]).output()?;
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
        .args(["-n", "50000", "-c", "256", "-t", "10"])
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
    println!("[{lang}] {url}");
    run_hey(&[&url], false)?;

    // Third run, for stats
    if let Some(content) = run_hey(&["-o", "csv", &url], true).unwrap() {
        CSV_PATTERN.with(|re| {
            let values = content
                .split('\n')
                .filter_map(|line| {
                    let mut result = None;
                    if let Some(captures) = re.captures(line) {
                        if let Some(m) = captures.name("responseTime") {
                            let double_value: f64 = m.as_str().parse().unwrap();
                            result = Some(double_value * 1000.0)
                        }
                    }
                    result
                })
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
        log(&format!("Killing {pid} process tree..."));
    }
    kill(pid);
    Ok((index_values, pattern_values))
}

fn draw<DB: DrawingBackend>(dataset: Vec<(String, &str, Quartiles)>, backend: DB) -> UnitResult
where
    DB::ErrorType: 'static,
{
    let lang_list: Vec<_> = dataset
        .iter()
        .unique_by(|x| x.0.clone())
        .sorted_by(|a, b| b.2.median().partial_cmp(&a.2.median()).unwrap())
        .map(|x| x.0.clone())
        .collect();

    let mut colors = (0..).map(Palette99::pick);
    let mut offsets = (-10..).step_by(20);
    let mut series = BTreeMap::new();
    for x in dataset.iter() {
        let entry = series
            .entry(x.1)
            .or_insert_with(|| (Vec::new(), colors.next().unwrap(), offsets.next().unwrap()));
        entry.0.push((x.0.clone(), &x.2));
    }

    let values: Vec<f32> = dataset.iter().flat_map(|x| x.2.values().to_vec()).collect();
    let values_range = fitting_range(values.iter());

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(100)
        .build_cartesian_2d(0.0..values_range.end + 1.0, lang_list[..].into_segmented())?;

    chart
        .configure_mesh()
        .x_desc("Response, ms")
        .y_desc("Language")
        .y_labels(lang_list.len())
        .light_line_style(WHITE)
        .draw()?;

    for (label, (values, style, offset)) in &series {
        chart
            .draw_series(values.iter().map(|x| {
                Boxplot::new_horizontal(SegmentValue::CenterOf(&x.0), x.1)
                    .width(10)
                    .whisker_width(0.5)
                    .style(style)
                    .offset(*offset)
            }))?
            .label(*label)
            .legend(move |(x, y)| Rectangle::new([(x, y - 7), (x + 12, y + 5)], style.filled()));
    }
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(BLACK.mix(0.5))
        .legend_area_size(22)
        .draw()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lang_cmds = BTreeMap::new();
    lang_cmds.insert(
        "go",
        Cmd {
            title: "Go",
            build: Box::new(|| {
                pexec(Command::new("go").args(["build", "-o", "go/build/main", "go/main.go"]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("go/build/main"))),
        },
    );
    lang_cmds.insert(
        "rust_tide",
        Cmd {
            title: "Rust/Tide",
            build: Box::new(|| {
                pexec(Command::new("cargo").args([
                    "build",
                    "--manifest-path",
                    "rust/tide/Cargo.toml",
                    "--release",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("rust/tide/target/release/tide-test"))),
        },
    );
    lang_cmds.insert(
        "rust_warp",
        Cmd {
            title: "Rust/warp",
            build: Box::new(|| {
                pexec(Command::new("cargo").args([
                    "build",
                    "--manifest-path",
                    "rust/warp/Cargo.toml",
                    "--release",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("rust/warp/target/release/warp-test"))),
        },
    );
    lang_cmds.insert(
        "rust_actix",
        Cmd {
            title: "Rust/Actix",
            build: Box::new(|| {
                pexec(Command::new("cargo").args([
                    "build",
                    "--manifest-path",
                    "rust/actix-web/Cargo.toml",
                    "--release",
                ]))
            }),
            run: Box::new(|| {
                pspawn(&mut Command::new(
                    "rust/actix-web/target/release/actix-web-test",
                ))
            }),
        },
    );
    lang_cmds.insert(
        "rust_hyper",
        Cmd {
            title: "Rust/hyper",
            build: Box::new(|| {
                pexec(Command::new("cargo").args([
                    "build",
                    "--manifest-path",
                    "rust/hyper/Cargo.toml",
                    "--release",
                ]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("rust/hyper/target/release/hyper-test"))),
        },
    );
    lang_cmds.insert(
        "scala",
        Cmd {
            title: "Scala/Akka",
            build: Box::new(|| {
                pexec(Command::new("make").args(["-C", "scala", "clean", "target/library.jar"]))
            }),
            run: Box::new(|| {
                let cp = format!(
                    "scala/target/library.jar:{}",
                    fs::read_to_string("scala/target/classpath.line")
                        .unwrap()
                        .trim()
                );
                pspawn(Command::new("scala").args(["-cp", &cp, "lite.WebServer"]))
            }),
        },
    );
    lang_cmds.insert(
        "java",
        Cmd {
            title: "Java/Spring Boot",
            build: Box::new(|| {
                pexec(Command::new("make").args(["-C", "java", "clean", "target/library.jar"]))
            }),
            run: Box::new(|| {
                let cp = format!(
                    "java/target/library.jar:{}",
                    fs::read_to_string("java/target/classpath.line").unwrap()
                );
                pspawn(Command::new("java").args([
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
            run: Box::new(|| pspawn(Command::new("node").args(["nodejs/main.js"]))),
        },
    );
    lang_cmds.insert(
        "crystal",
        Cmd {
            title: "Crystal",
            build: Box::new(|| {
                pexec(Command::new("crystal").args([
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
                pexec(Command::new("cargo").args([
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
            run: Box::new(|| pspawn(Command::new("pypy3").args(["python/twist.py"]))),
        },
    );
    lang_cmds.insert(
        "php",
        Cmd {
            title: "PHP/Swoole",
            build: Box::new(|| Ok(())),
            run: Box::new(|| {
                pspawn(Command::new("php").args([
                    "-c",
                    "php/swoole/php.ini",
                    "php/swoole/main.php",
                ]))
            }),
        },
    );
    lang_cmds.insert(
        "cpp",
        Cmd {
            title: "C++/Boost.Beast",
            build: Box::new(|| {
                pexec(Command::new("make").args(["-C", "cpp", "clean", "target/server"]))
            }),
            run: Box::new(|| pspawn(&mut Command::new("cpp/target/server"))),
        },
    );

    let default_file = "result.svg";
    let matches = clap::Command::new("Simple Web Benchmark runner")
        .version(crate_version!())
        .override_usage("cargo run --manifest-path suite/Cargo.toml -- [FLAGS] [OPTIONS] <lang>...")
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .value_name("file")
                .help("Sets an image file to generate (PNG/SVG/TSV are supported)")
                .default_value(default_file)
                .num_args(0..=1),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Enables the verbose output"),
        )
        .arg(
            Arg::new("lang")
                .index(1)
                .action(clap::ArgAction::Append)
                .required(true)
                .help("Sets the languages to test ('all' for all)"),
        )
        .after_help(&format!(
            "The following languages are supported: {}.",
            lang_cmds.keys().join(", ")
        ))
        .get_matches();
    let verbose = matches.contains_id("verbose");
    let mut langs: Vec<String> = matches
        .get_many::<String>("lang")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    if langs.iter().any(|x| x == "all") {
        langs = lang_cmds.keys().map(|key| key.to_string()).collect();
    }
    let file = matches.get_one::<String>("out").unwrap();
    let ext = Path::new(file)
        .extension()
        .ok_or_else(|| Box::new(errors::UnknownFileTypeError {}))?;
    let save_for_print = ext.to_str().map_or(false, |x| x == "tsv");

    let mut dataset = Vec::new();
    let mut dataset_for_print = Vec::new();
    for (lang, lang_cmd) in &lang_cmds {
        if !langs.iter().any(|x| x == lang) {
            continue;
        }

        while kill_processes().unwrap() {
            // kill until all died
        }
        let (index_values, pattern_values) = run(lang_cmd, verbose).unwrap();
        if save_for_print {
            for x in index_values {
                dataset_for_print.push(format!(
                    "{}\t{}\t{}",
                    String::from(lang_cmd.title),
                    INDEX,
                    x
                ));
            }
            for x in pattern_values {
                dataset_for_print.push(format!(
                    "{}\t{}\t{}",
                    String::from(lang_cmd.title),
                    PATTERN,
                    x
                ));
            }
        } else {
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
    }

    match ext.to_str().unwrap() {
        "tsv" => fs::write(file, dataset_for_print.join("\n"))?,
        "svg" => draw(dataset, SVGBackend::new(file, (480, 640)))?,
        _ => draw(dataset, BitMapBackend::new(file, (480, 640)))?,
    }
    Ok(())
}
