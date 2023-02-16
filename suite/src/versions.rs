use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::process::Command;

mod errors;

type StringResult = Result<String, Box<dyn Error>>;

thread_local! {
    static LDC_PATTERN: Regex = Regex::new(r"version\s+(.*)\s+\(").unwrap();
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
    let mut table = vec!["|===".to_string(), "| Language | Version".to_string()];

    let mut langs: BTreeMap<&str, Box<dyn Fn() -> StringResult>> = BTreeMap::new();
    langs.insert(
        "C{pp}/g{pp}",
        Box::new(|| pexec(Command::new("g++").args(["-dumpfullversion"]))),
    );
    langs.insert(
        "Rust",
        Box::new(|| {
            let text = pexec(Command::new("rustc").args(["--version"]))?;
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
            pexec(Command::new("go").args(["run", &cat("go.go", prog)?]))
        }),
    );
    langs.insert(
        "Scala",
        Box::new(|| {
            let output = Command::new("scala").args(["-version"]).output()?;
            let text = String::from_utf8(output.stderr)?;
            to_result(text.split_whitespace().nth(4).map(String::from))
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
            pexec(Command::new("java").args([&cat("java.java", prog)?]))
        }),
    );
    langs.insert(
        "Node.js",
        Box::new(|| pexec(Command::new("node").args(["-e", "console.log(process.version)"]))),
    );
    langs.insert(
        "Crystal",
        Box::new(|| pexec(Command::new("crystal").args(["eval", "puts Crystal::VERSION"]))),
    );
    langs.insert(
        "PyPy",
        Box::new(|| {
            let prog = r#"
import platform, sys
pypy = "%d.%d.%d-%s%d" % sys.pypy_version_info
print("%s for Python %s" % (pypy, platform.python_version()))
"#;
            pexec(Command::new("pypy3").args([&cat("pypy.py", prog)?]))
        }),
    );
    langs.insert(
        "PHP",
        Box::new(|| pexec(Command::new("php").args(["-r", "echo phpversion();"]))),
    );

    for (name, version_lambda) in &langs {
        eprint!("Fetching {name} version... ");
        match version_lambda() {
            Ok(version) => {
                table.push(format!("\n| {}\n| {}", name, &version));
                eprintln!("ok");
            }
            Err(e) => {
                eprintln!("fail ({e})");
            }
        }
    }
    eprintln!("\n");
    table.push("|===".to_string());
    println!("{}", table.join("\n"));
    Ok(())
}
