use rustyline::{DefaultEditor, error::ReadlineError};

// Minimal Command Dispatcher, a very minimal shell-like environment for running commands.
pub fn mcd() -> Result<(), String> {
    let mut input = match DefaultEditor::new() {
        Ok(e) => e,
        Err(err) => return Err(err.to_string())
    };

    loop {
        match std::env::current_dir() {
            Err(_) => {
                eprintln!("There was an error while getting the Current Working Directory!\nAttempting to fall back to Home...");
                match std::env::home_dir() {
                    None => return Err("Getting home failed! Something has gone wrong and MCD is exitting now.".to_string()),
                    Some(home) => {
                        match std::env::set_current_dir(home) {
                            Err(err) => return Err(err.to_string()),
                            Ok(_) => println!("Fallback to Home Successful!") 
                        }
                    }
                };
            }
            Ok(cwd) => {
                match cwd.to_str() {
                    Some(d) => println!("{}", d),
                    None => continue
                }
            }
        };

        let rawline = match input.readline("MCD >> ") {
            Ok(t) => t,
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue
            }

            Err(e) => return Err(e.to_string())
        };

        let args: Vec<&str> = rawline.split_whitespace().collect();
        match args[0] {
            "" => { /* Empty input, do nothing. */ }
            "exit" => { println!("Bye!"); return Ok(()) }
            "cd" => {
                if args.len() < 2 {
                    println!("Usage: cd <target>");
                    continue
                }

                if let Err(err) = std::env::set_current_dir(args[1]) {
                    eprintln!("{}", err.to_string())
                }
            }
        
            _ => {
                match std::process::Command::new(args[0]).args(&args[1..]).spawn() {
                    Ok(mut child) => if let Err(err) = child.wait() { eprintln!("{}", err.to_string()) }
                    Err(err) => eprintln!("{}", err.to_string())
                }
            }
        }
    }
}
