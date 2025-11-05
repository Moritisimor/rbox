use std::io::{Write, stdin, stdout};

// Minimal Command Dispatcher, a very minimal shell-like environment for running commands.
pub fn mcd() -> Result<(), String>{
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
                    Some(d) => println!("\n{}", d),
                    None => continue
                }
            }
        };

        print!("MCD >> ");
        if let Err(err) = stdout().flush() {
            println!("Error: {}", err);
            continue
        }

        let mut input = String::new();
        if let Err(err) = stdin().read_line(&mut input) {
            println!("Error: {}", err);
            continue
        }

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        match args[0] {
            "" => { /* Empty input, do nothing. */ }
            "exit" => { println!("Bye!"); return Ok(()) }
            "cd" => {
                if args.len() < 2 {
                    println!("Usage: cd <target>");
                    continue
                }

                if let Err(err) = std::env::set_current_dir(args[1]) {
                    eprintln!("Error: {}", err)
                }
            }
        
            _ => {
                match std::process::Command::new(args[0]).args(&args[1..]).spawn() {
                    Ok(mut child) => if let Err(err) = child.wait() {
                        eprintln!("Error: {}", err)
                    }
                    Err(err) => { eprintln!("An error has occurred! Error: {}", err) }
                }
            }
        }
    }
}
