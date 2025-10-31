use crate::internals;

pub fn crtd(args: Vec<String>) {
    if args.len() < 2 { println!("Usage: crtd <directory>"); return }

    match std::fs::create_dir(args[2].clone()) {
        Ok(_) => { /* Success! */ }
        Err(err) => { println!("Could not create Directory!\nError: {}", err) }
    }
}

pub fn ls(args: Vec<String>) {
    let mut target = String::from(".");
    let mut showhidden = false;

    for i in &args[2..] {
        if !i.starts_with("-") {
            target = i.clone()
        } else {
            match i.trim() {
                "-a" => showhidden = true,
                _ => { println!("Unknown flag: {}", i); return }
            }
        }
    }

    match std::fs::read_dir(target) {
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
                    continue;
                }
                println!("- {} ({})", entryname, internals::getentrytype(readentry))
            }
        }

        Err(err) => println!("Could not read Directory!\nError: {}", err)
    }
}
