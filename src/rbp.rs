use std::{fs, process};

pub fn rbp(args: &[String]) -> Result<(), String> {
    let file = match args.get(0) {
        None => { println!("Usage: rbp <File>"); return Ok(()) }
        Some(f) => f
    };

    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };

    let mut i = 0;
    for l in content.lines() {
        i += 1;
        let parts: Vec<&str> = l.split_whitespace().collect();
        let cmd = process::Command::new(parts[0]).args(&parts[1..]).spawn();
        let mut handle = match cmd {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Batch-Execution stopped at line {} due to an error.", i);
                return Err(e.to_string());
            }
        };

        if let Err(e) = handle.wait() { return Err(e.to_string())};
    }

    Ok(())
}
