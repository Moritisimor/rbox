// Create File
pub fn crtf(args: &[String]) -> Result<(), String> {
    if args.len() < 1 { println!("Usage: ctrf <file>"); return Ok(()) }
    // Not really an error since this may have just been called for the help.

    let mut files: Vec<&str> = vec![];
    for arg in args {
        if !arg.starts_with("-") { files.push(&arg) }
    }
    
    for file in files {
        if let Err(err) = std::fs::File::create_new(file) { return Err(err.to_string()) }
    }

    Ok(())
}

// Remove File
pub fn rmf(args: &[String]) -> Result<(), String> {
    if args.len() < 1 { println!("Usage: rmf <file>"); return Ok(()) }

    let mut files: Vec<&str> = vec![];
    for arg in args {
        if !arg.starts_with("-") { files.push(&arg); }
    }

    for file in files {
        if let Err(err) = std::fs::remove_file(file) { return Err(err.to_string()); }
    }

    Ok(())
}

// Read File
pub fn rdf(args: &[String]) -> Result<(), String> {
    if args.len() < 1 {
        println!("Usage: rdf <file>"); 
        return Ok(()) 
    }

	let mut erroccur = false;
	let mut files: Vec<&str> = vec![];
    for arg in args {
        if !arg.starts_with("-") { files.push(arg); }
    }

	let fileslen = files.len();
    for f in files {
		if fileslen > 1 { println!("[{}]", f) }

        match std::fs::read_to_string(f) {
            Err(err) => { 
				eprintln!("Opening {} failed! Error: {}\n", f, err); 
				erroccur = true;
				continue 
			}

            Ok(content) => {
                if content.is_empty() {
                    println!("(Empty file)")
                }
                println!("{}", content) // It's empty anyway
            }
        }
    }

	if erroccur { 
        Err("Reading one or more files failed.".to_string()) 
    } else {
        Ok(())
    }
}

// Move File
pub fn mve(args: &[String]) -> Result<(), String> {
    let file = match args.get(0) {
        None => return Err(String::from("Expected 2 arguments, got 0")),
        Some(f) => f
    };

    let new_name = match args.get(1) {
        None => return Err(String::from("Expected 2 arguments, got 1")),
        Some(n) => n
    };

    if let Err(err) = std::fs::rename(file, new_name) {
        return Err(err.to_string())    
    }
    
    Ok(())
}

pub fn cpy(args: &[String]) -> Result<(), String> {
    let og = match args.get(0) {
        None => return Err(String::from("Expected 2 arguments, got 0")),
        Some(o) => o
    };

    let new = match args.get(1) {
        None => return Err(String::from("Expected 2 arguments, got 1")),
        Some(n) => n
    };

    if let Err(err) = std::fs::copy(og, new) {
        return Err(err.to_string())
    }
    
    Ok(())
}

