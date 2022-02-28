use dirs;
use lexopt;
use std::collections::HashMap;
use std::fs;

mod shell;

const USAGE: &str = "Usage:
  add | a <name>    Bind current directory to name (default: base name)
  list | l          Lists the currently installed shortcuts
  init              Prints out the Bash integration code
  help | --help     Prints out this help message";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    if let Args::Help = args {
        println!("{}", USAGE);
        std::process::exit(0);
    };

    if let Args::Init(sh) = args {
        match sh {
            shell::Shell::Bash => {
                println!("{}", shell::Shell::Bash);
                std::process::exit(0);
            }
            _ => todo!(),
        }
    }

    let gotorc = match dirs::config_dir() {
        Some(mut d) => {
            d.push("goto");
            // The directory might already exist, don't bother checking result
            let _ = fs::create_dir(&d);
            d.push("gotorc");
            d
        }
        None => {
            let mut d = dirs::home_dir().ok_or("no home dir")?;
            d.push(".gotorc");
            d
        }
    };

    let rc_file = fs::OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(&gotorc)?;

    let file = fs::read_to_string(&gotorc)?;
    let jump_map = file
        .lines()
        .map(|l| l.split_once(":"))
        .collect::<Option<HashMap<_, _>>>()
        .ok_or("no jump file")?;

    if let Args::List = args {
    }

    Ok(())
}

#[derive(Debug)]
enum Args {
    Dir(String),
    List,
    Add(String),
    Init(shell::Shell),
    Help,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut parser = lexopt::Parser::from_env();
    if let Some(arg) = parser.next()? {
        match arg {
            Value(val) => match val.to_str().ok_or("not a string")? {
                "l" | "list" => Ok(Args::List),
                "a" | "add" => {
                    if let Some(Value(dir)) = parser.next()? {
                        Ok(Args::Add(dir.into_string()?))
                    } else {
                        println!(r#"Error: "add" requires a shortcut name"#);
                        std::process::exit(1);
                    }
                }
                "rm" | "remove" => todo!(),
                "init" => Ok(Args::Init(shell::Shell::Bash)),
                "help" => Ok(Args::Help),
                dir => Ok(Args::Dir(dir.into())),
            },
            Long("help") => Ok(Args::Help),
            _ => Err(arg.unexpected()),
        }
    } else {
        Ok(Args::Help)
    }
}
