use std::io::Read;

pub fn crtf(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: ctrf <file>"); return }
    
    match std::fs::File::create_new(args[2].clone()) {
        Ok(_) => { /* Success */ }
        Err(err) => println!("Could not create file!\nError: {}", err)
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
        match std::fs::File::open(f) {
            Err(err) => { println!("Opening {} failed!\nError: {}", f, err); return }

            Ok(mut openfile) => {
                let mut buffer = String::new();
                match openfile.read_to_string(&mut buffer) {
                    Err(err) => { println!("Reading to internal buffer failed!\nError: {}", err); return }

                    Ok(_) => {
                        println!("{}", buffer)
                    }
                }
            }
        }
    }
}
