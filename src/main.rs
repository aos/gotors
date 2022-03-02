use dirs;
use lexopt;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

mod shell;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const SEPARATOR: &str = ":";
const USAGE: &str = "Usage:
  l | list              Lists the currently installed shortcuts
  a | add <name>        Bind current directory to name (default: basename)
  rm | remove <name>    Remove the specified shortcut
  init                  Prints out shell integration code
  help | --help         Prints out this help message";

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let args = parse_args(&cwd)?;
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

    let mut jump_map = {
        let mut rc_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&gotorc)?;
        let mut s = String::new();
        rc_file.read_to_string(&mut s)?;
        s.lines()
            .map(|l| {
                l.split_once(SEPARATOR)
                    .map(|(k, v)| (k.to_string(), v.to_string()))
            })
            .collect::<Option<HashMap<_, _>>>()
            .ok_or("no jump_map")?
    };

    match args {
        Args::List => {
            if jump_map.len() > 0 {
                println!("Shortcuts available:");
                for (k, v) in jump_map {
                    println!("{:5} -> {}", k, v);
                }
            } else {
                println!("No shortcuts added.");
            }
        }
        Args::Add(shortcut) => {
            let mut rc_file = fs::OpenOptions::new()
                .read(true)
                .append(true)
                .open(&gotorc)?;
            writeln!(rc_file, "{}{}{}", shortcut, SEPARATOR, cwd.display())?;
            println!("Added shortcut: {} -> {}", shortcut, cwd.display());
        }
        Args::Remove(shortcut) => match jump_map.remove_entry(&*shortcut) {
            Some((old_shortcut, old_path)) => {
                let mut rc_file = fs::OpenOptions::new()
                    .truncate(true)
                    .write(true)
                    .open(&gotorc)?;
                for (k, v) in jump_map {
                    writeln!(rc_file, "{}{}{}", k, SEPARATOR, v)?;
                }
                println!("Removed shortcut: {} -> {}", old_shortcut, old_path);
            }
            None => {
                eprintln!("Error: shortcut does not exist.");
                std::process::exit(1);
            }
        },
        Args::Dir(shortcut) => {
            print!(
                "{}",
                jump_map
                    .get(shortcut.as_str())
                    .ok_or("shortcut not found")?
            );
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[derive(Debug)]
enum Args {
    Dir(String),
    List,
    Add(String),
    Remove(String),
    Init(shell::Shell),
    Help,
}

fn parse_args(cwd: &std::path::PathBuf) -> Result<Args> {
    use lexopt::prelude::*;

    let mut parser = lexopt::Parser::from_env();
    if let Some(arg) = parser.next()? {
        match arg {
            Value(val) => match val.to_str().ok_or("not a string")? {
                "l" | "list" => Ok(Args::List),
                "a" | "add" => {
                    if let Some(Value(dir)) = parser.next()? {
                        let d = dir
                            .into_string()
                            .map_err(|e| lexopt::Error::NonUnicodeValue(e))?;
                        Ok(Args::Add(d))
                    } else {
                        // default to basename of directory
                        let basename = cwd
                            .file_name()
                            .and_then(|f| Some(f.to_str()?.to_owned()))
                            .ok_or("no basename")?;
                        Ok(Args::Add(basename))
                    }
                }
                "rm" | "remove" => {
                    if let Some(Value(shortcut)) = parser.next()? {
                        let s = shortcut
                            .into_string()
                            .map_err(|e| lexopt::Error::NonUnicodeValue(e))?;
                        Ok(Args::Remove(s))
                    } else {
                        eprintln!("Error: shortcut not specified.");
                        std::process::exit(1);
                    }
                }
                "init" => Ok(Args::Init(shell::Shell::Bash)),
                "help" => Ok(Args::Help),
                dir => Ok(Args::Dir(dir.into())),
            },
            Long("help") => Ok(Args::Help),
            _ => Err(Box::new(arg.unexpected())),
        }
    } else {
        Ok(Args::Help)
    }
}
