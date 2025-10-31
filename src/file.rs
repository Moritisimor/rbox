pub fn crtf(args: Vec<String>) {
    if args.len() < 3 { println!("Usage: ctrf <file> "); return }

    match std::fs::File::create_new(args[2].clone()) {
        Ok(_) => { /* Success */ }
        Err(err) => println!("Could not create file!\nError: {}", err)
    }
}
