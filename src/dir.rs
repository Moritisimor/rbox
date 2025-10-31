use std::fs::DirEntry;

fn getentrytype(entry: DirEntry) -> String {
    match entry.file_type() {
        Ok(readentry) => {
            if readentry.is_dir() {
                "Dir".to_string()
            } else if readentry.is_file() {
                "File".to_string()
            } else {
                "Symlink".to_string()
            }
        }
        Err(_) => {
            "Unknown".to_string()
        }
    }
}

pub fn crtd(args: Vec<String>) {
    if args.len() < 2 { println!("Usage: crtd <directory>"); return }

    match std::fs::create_dir(args[2].clone()) {
        Ok(_) => { /* Success! */ }
        Err(err) => { println!("Could not create Directory!\nError: {}", err) }
    }
}

pub fn ls(args: Vec<String>) {
    let mut target = String::from(".");
    if args.len() > 2 {
        target = args[2].clone()
    }

    match std::fs::read_dir(target) {
        Ok(entries) => {
            for entry in entries {
                let readentry = entry.unwrap(); // Better error handling coming soon trust
                let entryname = readentry.file_name().into_string().unwrap();

                if !entryname.starts_with(".") {
                    println!("- {} ({})", entryname, getentrytype(readentry))
                }
            }
        }

        Err(err) => println!("Could not read Directory!\nError: {}", err)
    }
}
