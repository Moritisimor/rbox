// Basically pwd but it reads like plain english
pub fn whereami() -> Result<(), String>{
    match std::env::current_dir() {
        Err(err) => Err(err.to_string()),
        Ok(wd) => { 
            if let Some(dir) = wd.to_str() {
                println!("{}", dir);
                return Ok(())
            }

            Err("Parsing WD-Path Buffer to String failed!".to_string())
        }
    }
}
