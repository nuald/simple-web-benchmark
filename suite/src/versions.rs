use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::process::Command;

mod errors;

type StringResult = Result<String, Box<dyn Error>>;

thread_local! {
    static LDC_PATTERN: Regex = Regex::new(r"version\s+(.*)\s+\(").unwrap();
}

fn pad(width: usize, text: &str, fill: char) -> String {
    let trimmed = text.trim();
    let fill_len = width - trimmed.len();
    let fill_str: String = (0..fill_len).map(|_| fill).collect();
    trimmed.to_owned() + &fill_str
}

fn lpad_with_fill(text: &str, fill: char) -> String {
    pad(12, text, fill)
}

fn lpad(text: &str) -> String {
    lpad_with_fill(text, ' ')
}

fn rpad_with_fill(text: &str, fill: char) -> String {
    pad(31, text, fill)
}

fn rpad(text: &str) -> String {
    rpad_with_fill(text, ' ')
}

fn pexec(cmd: &mut Command) -> StringResult {
    let output = cmd.output()?;
    let str = String::from_utf8(output.stdout)?;
    Ok(str)
}

fn touch(filename: &str) -> StringResult {
    let mut dir = std::env::temp_dir();
    dir.push(filename);
    to_result(dir.to_str().map(String::from))
}

fn cat(filename: &str, content: &str) -> StringResult {
    let path = touch(filename);
    if let Ok(valid_path) = &path {
        std::fs::write(valid_path, content)?;
    }
    path
}

fn to_result(opt: Option<String>) -> StringResult {
    opt.ok_or_else(|| Box::new(errors::ValueIsEmptyError {}) as Box<dyn Error>)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut table = vec![
        format!("| {} | {} |", lpad("Language"), rpad("Version")),
        format!(
            "| {} | {} |",
            lpad_with_fill("-", '-'),
            rpad_with_fill("-", '-')
        ),
    ];

    let mut langs: BTreeMap<&str, Box<dyn Fn() -> StringResult>> = BTreeMap::new();
    langs.insert(
        "C++/g++",
        Box::new(|| {
            pexec(Command::new("g++").args(&["-dumpfullversion"]))
        }),
    );
    langs.insert(
        "Rust",
        Box::new(|| {
            let text = pexec(Command::new("rustc").args(&["--version"]))?;
            to_result(text.split_whitespace().nth(1).map(String::from))
        }),
    );
    langs.insert(
        "Go",
        Box::new(|| {
            let prog = r#"
package main
import (
  "fmt"
  "runtime"
)
func main() {
  fmt.Printf(runtime.Version())
}
"#;
            pexec(Command::new("go").args(&["run", &cat("go.go", prog)?]))
        }),
    );
    langs.insert(
        "Scala",
        Box::new(|| {
            let output = Command::new("scala").args(&["-version"]).output()?;
            let text = String::from_utf8(output.stderr)?;
            to_result(text.split_whitespace().nth(3).map(String::from))
        }),
    );
    langs.insert(
        "Java",
        Box::new(|| {
            let prog = r#"
class Test {
  public static void main(String[] argv) {
    System.out.print(System.getProperty("java.version"));
  }
}
"#;
            pexec(Command::new("java").args(&[&cat("java.java", prog)?]))
        }),
    );
    langs.insert(
        "Node.js",
        Box::new(|| pexec(Command::new("node").args(&["-e", "console.log(process.version)"]))),
    );
    langs.insert(
        "LDC",
        Box::new(|| {
            let xf = format!("-Xf={}", touch("ldc.json")?);
            let output = pexec(Command::new("ldc2").args(&["-v", "-X", &xf, "-Xi=compilerInfo"]))?;
            LDC_PATTERN.with(|re| {
                to_result(
                    output
                        .lines()
                        .nth(1)
                        .map(|x| re.captures(x))
                        .flatten()
                        .map(|caps| caps.get(1))
                        .flatten()
                        .map(|v| v.as_str().to_string()),
                )
            })
        }),
    );
    langs.insert(
        "DMD",
        Box::new(|| {
            let path = touch("dmd.json")?;
            let xf = format!("-Xf={}", path);
            let _ = pexec(Command::new("dmd").args(&["-X", &xf, "-Xi=compilerInfo"]))?;
            let data = std::fs::read_to_string(path)?;
            let v: Value = serde_json::from_str(&data)?;
            to_result(v["compilerInfo"]["version"].as_str().map(String::from))
        }),
    );
    langs.insert(
        "Crystal",
        Box::new(|| pexec(Command::new("crystal").args(&["eval", "puts Crystal::VERSION"]))),
    );
    langs.insert(
        "PyPy",
        Box::new(|| {
            let prog = r#"
import platform, sys
pypy = "%d.%d.%d-%s%d" % sys.pypy_version_info
print("%s for Python %s" % (pypy, platform.python_version()))
"#;
            pexec(Command::new("pypy3").args(&[&cat("pypy.py", prog)?]))
        }),
    );
    langs.insert(
        "PHP",
        Box::new(|| pexec(Command::new("php").args(&["-r", "echo phpversion();"]))),
    );

    for (name, version_lambda) in &langs {
        eprint!("{}", format!("Fetching {} version... ", name));
        match version_lambda() {
            Ok(version) => {
                table.push(format!("| {} | {} |", lpad(name), rpad(&version)));
                eprintln!("ok");
            }
            Err(e) => {
                eprintln!("fail ({})", e);
            }
        }
    }
    eprintln!("\n");
    println!("{}", table.join("\n"));
    Ok(())
}
