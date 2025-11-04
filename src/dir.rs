use crate::internals;

// Create Directory
pub fn crtd(args: &[String]) {
    if args.len() < 1 { println!("Usage: crtd <directory>"); return }
    let mut dirs: Vec<&str> = vec![];

    for arg in args {
        if !arg.starts_with("-") {
            dirs.push(&arg)
        }
    } 

    for dir in dirs {
        if let Err(err) = std::fs::create_dir(dir) {
            println!("Could not create Directory '{}'!\nError: {}", dir, err);
            return
        }
    }
}

// List entries in a directory
pub fn ls(args: &[String]) {
    let mut target = ".";
    let mut showhidden = false;

    for arg in args {
        if !arg.starts_with("-") {
            target = arg
        } else {
            match arg.trim() {
                "-a" => showhidden = true,
                _ => { println!("Unknown flag: {}", arg); return }
            }
        }
    }

    match std::fs::read_dir(target) {
        Err(err) => println!("Could not read Directory!\nError: {}", err),
        Ok(entries) => {
            for entry in entries {
                let readentry = match entry {
                    Ok(good) => good,
                    Err(bad) => { println!("Error while reading entry!\nError: {}", bad); return }
                };

                let entryname = match readentry.file_name().into_string() {
                    Ok(good) => good,
                    Err(_) => { println!("Error while parsing entry to string!"); return }
                };

                if entryname.starts_with(".") && !showhidden {
                    continue
                }
                println!("- {} ({})", entryname, internals::getentrytype(readentry))
            }
        }
    }
}
