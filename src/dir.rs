pub fn crtd(args: Vec<String>) {
    if args.len() < 2 { println!("Usage: crtd <directory>"); return }

    match std::fs::create_dir(args[2].clone()) {
        Ok(_) => { /* Success! */ }
        Err(err) => { println!("Could not create Directory!\nError: {}", err) }
    }
}
