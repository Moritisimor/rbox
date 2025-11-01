// Create File
pub fn crtf(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: ctrf <file>"); return }
    let mut files: Vec<&str> = vec![];

    for arg in &args[2..] {
        if !arg.starts_with("-") {
            files.push(&arg)
        }
    }
    
    for file in files {
        if let Err(err) = std::fs::File::create_new(file) {
            println!("Could not create file!\nError: {}", err)
        }
    }
}

// Read File
pub fn rdf(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: rdf <file>"); return }
	let mut erroccur = false;

	let mut files: Vec<&str> = vec![];
    for arg in &args[2..] {
        if !arg.starts_with("-") {
            files.push(arg);
        }
    }

	let fileslen = files.len();
    for f in files {
		if fileslen > 1 {
			println!("[{}]", f)
		}

        match std::fs::read_to_string(f) {
            Err(err) => { 
				eprintln!("Opening {} failed!\nError: {}\n", f, err); 
				erroccur = true;
				continue 
			}

            Ok(content) => {
                if content.is_empty() {
                    println!("(Empty file)")
                }
                println!("{}", content)
            }
        }
    }

	if erroccur { std::process::exit(1) }
}
