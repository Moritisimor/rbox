use crate::internals;

pub fn crtd(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: crtd <directory>"); return }
    let mut dirs: Vec<&str> = vec![];

    for arg in &args[2..] {
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

pub fn ls(args: Vec<String>) {
    let mut target = ".";
    let mut showhidden = false;

    for i in &args[2..] {
        if !i.starts_with("-") {
            target = i
        } else {
            match i.trim() {
                "-a" => showhidden = true,
                _ => { println!("Unknown flag: {}", i); return }
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
