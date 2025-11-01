// Basically pwd but it reads like plain english
pub fn whereami() {
    match std::env::current_dir() {
        Err(err) => eprintln!("Could not get current working directory!\nError: {}", err),
        Ok(wd) => println!("{}", wd.to_str().expect("Could not Convert directory to string!"))
    }
}
