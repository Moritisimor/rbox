use crate::internals;

// Create Directory
pub fn crtd(args: &[String]) -> Result<(), String>{
    if args.len() < 1 { println!("Usage: crtd <directory>"); return Ok(()) }
    let mut dirs: Vec<&str> = vec![];

    for arg in args {
        if !arg.starts_with("-") {
            dirs.push(&arg)
        }
    } 

    for dir in dirs {
        if let Err(err) = std::fs::create_dir(dir) { return Err(err.to_string()) }
    }

    Ok(())
}

// List entries in a directory
pub fn ls(args: &[String]) -> Result<(), String>{
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
