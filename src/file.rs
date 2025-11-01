pub fn crtf(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: ctrf <file>"); return }
    let mut files: Vec<&str> = vec![];

    for arg in &args {
        if !arg.starts_with("-") {
            files.push(&arg);
        }
    }
    
    for file in files {
        if let Err(err) = std::fs::File::create_new(file) {
            println!("Could not create file!\nError: {}", err)
        }
    }
}

pub fn rdf(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: rdf <file>"); return }

    let mut files: Vec<&str> = vec![];
    for arg in &args[2..] {
        if !arg.starts_with("-") {
            files.push(arg);
        }
    }

    for f in files {
        match std::fs::read_to_string(f) {
            Err(err) => { println!("Opening {} failed!\nError: {}", f, err); return }

            Ok(content) => {
                if content.is_empty() {
                    println!("(Empty file)")
                }
                println!("{}", content)
            }
        }
    }
}
