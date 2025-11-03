use std::io::{Write, stdin, stdout};

// Minimal Command Dispatcher, a very minimal shell-like environment for running commands.
pub fn mcd() {
    let msg = "Error while getting Home directory!\nSomething has seriously gone wrong and MCD is shutting down now.";
    loop {
        match std::env::current_dir() {
            Err(_) => {
                eprintln!("There was an error while getting the Current Working Directory!\nAttempting to fall back to Home...");
                std::env::set_current_dir(std::env::home_dir().expect(msg)).expect(msg)
            }
            Ok(cwd) => {
                match cwd.to_str() {
                    Some(d) => println!("{}", d),
                    None => continue
                }
            }
        };

        print!("MCD >> ");
        if let Err(err) = stdout().flush() {
            println!("An error has occurred while flushing Stdout!\nError: {}", err);
            continue
        }

        let mut input = String::new();
        if let Err(err) = stdin().read_line(&mut input) {
            println!("An error has occurred while reading input from Stdin!\nError: {}", err);
            continue
        }

        let args: Vec<&str> = input.trim().split(" ").collect();
        match args[0] {
            "exit" => { println!("Bye!"); return }
            "cd" => {
                if args.len() < 2 {
                    println!("Usage: cd <target>");
                    continue
                }

                if let Err(err) = std::env::set_current_dir(args[1]) {
                    eprintln!("Error while changing directory!\nError: {}", err)
                }
            }
            
            "" => { /* Empty input, do nothing. */ }
            _ => {
                match std::process::Command::new(args[0]).args(&args[1..]).spawn() {
                    Ok(mut child) => if let Err(err) = child.wait() {
                        eprintln!("An error has occurred!\nError: {}", err)
                    }
                    Err(err) => { eprintln!("An error has occurred!\nError: {}", err) }
                }
            }
        }
    }
}
