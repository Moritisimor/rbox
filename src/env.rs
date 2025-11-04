// Basically pwd but it reads like plain english
pub fn whereami() -> Result<(), String>{
    match std::env::current_dir() {
        Err(err) => Err(err.to_string()),
        Ok(wd) => { 
            println!("{}", wd.to_str().expect("Could not Convert directory to string!"));
            Ok(())
        }
    }
}
