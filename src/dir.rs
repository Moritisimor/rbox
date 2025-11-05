use std::io::{Write, stdin, stdout};

use crate::internals::{self};

// Create Directory
pub fn crtd(args: &[String]) -> Result<(), String> {
    if args.len() < 1 { println!("Usage: crtd <directory>"); return Ok(()) }

    let mut dirs: Vec<&str> = vec![];
    for arg in args { 
        if !arg.starts_with("-") { dirs.push(&arg) } 
    } 

    for dir in dirs { 
        if let Err(err) = std::fs::create_dir(dir) { return Err(err.to_string()) } 
    }

    Ok(())
}

// Remove Directory
pub fn rmd(args: &[String]) -> Result<(), String> {
    if args.len() < 1 { println!("Usage: deld <directory>"); return Ok(()) }

    let mut dirs: Vec<&str> = vec![];
    for arg in args {
        if !arg.starts_with("-") { dirs.push(&arg) }
    }

    for dir in dirs {
        let empty = match std::fs::read_dir(dir) {
            Ok(entries) => entries.count() == 0,
            Err(err) => return Err(err.to_string())
        };

        if !empty {
            let mut ans = String::new();
            print!("Warning! {} is not empty! Delete recursively?\nY/N: ", dir);
            if let Err(err) = stdout().flush() { return Err(err.to_string()) }
            if let Err(err) = stdin().read_line(&mut ans) { return Err(err.to_string()) }

            if ans.trim().to_lowercase() != "y" {
                continue
            }

            if let Err(err) = std::fs::remove_dir_all(dir) { return Err(err.to_string()) }
        }
    }

    Ok(())
}

// List entries in a directory
pub fn ls(args: &[String]) -> Result<(), String> {
    let mut target = ".";
    let mut showhidden = false;

    for arg in args {
        if !arg.starts_with("-") {
            target = arg
        } else {
            match arg.trim() {
                "-a" => showhidden = true,
                _ => return Err("Unknown Flag!".to_string())
            }
        }
    }

    match std::fs::read_dir(target) {
        Err(err) => Err(err.to_string()),
        Ok(entries) => {
            for entry in entries {
                let readentry = match entry {
                    Ok(good) => good,
                    Err(bad) => return Err(bad.to_string())
                };

                let entryname = match readentry.file_name().into_string() {
                    Ok(good) => good,
                    Err(_) => return Err("Error while parsing entry to string!".to_string()) 
                };

                if entryname.starts_with(".") && !showhidden {
                    continue
                }
                println!("- {} ({})", entryname, internals::getentrytype(readentry))
            }

            Ok(())
        }
    }
}
