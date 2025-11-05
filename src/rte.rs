// RTE is a minimal text editor for use with RBox.
// This was originally made by RobertFlexx but has been adapted for use within RBox by Moritisimor.

use std::fs::{self, OpenOptions};
use std::io::{self, Write, stdout};
use std::path::PathBuf;

struct Editor {
    filename: PathBuf,
    lines: Vec<String>,
    dirty: bool,
}

fn print_help() {
    println!("Commands:");
    println!("=> r   --> Read all lines.");
    println!("=> r X --> Read line where X is the number of the line.");
    println!("=> a   --> Append lines. '.' on a line by itself to finish.");
    println!("=> e   --> Edit line.");
    println!("=> i   --> Insert line. This command inserts text at the index, incrementing the index of all following lines.");
    println!("=> d X --> Delete line where X is the number of the line.");
    println!("=> w   --> Write changes to file.");
    println!("=> q   --> Quit.");
    println!("=> q!  --> Force Quit.\n");
}

impl Editor {
    fn new(filename: PathBuf) -> io::Result<Editor> {
        let mut lines = Vec::new();
        if filename.exists() {
            let contents = fs::read_to_string(&filename)?;
            for line in contents.lines() {
                lines.push(line.to_owned())
            }
        }

        Ok(Editor { filename, lines, dirty: false })
    }

    fn read_lines(&self, line_num: Option<usize>) -> Result<(), String> {
        if let Some(n) = line_num {
            if n == 0 || n > self.lines.len() {
                eprintln!("No such line: {}", n);
                return Err(String::from("Line does not Exist!"))
            }
            println!("{} | {}", n, self.lines[n - 1]);
        } else {
            if self.lines.is_empty() {
                println!("[buffer is empty]")
            } else {
                for (idx, line) in self.lines.iter().enumerate() {
                    println!("{} | {}", idx + 1, line)
                }
            }
        }

        Ok(())
    }

    fn edit_line(&mut self, line: &str) -> Result<(), String> {
        println!("Edit line. Press Enter to apply changes.\n");
        self.read_lines(line.parse::<usize>().ok())?;

        println!("Original above ^");
        println!("(Leave blank to cancel) Edit below: ");
        print!("{} | ", line);
        if let Err(err) = io::stdout().flush() {
            eprintln!("Important: Reading from stdin failed!\nError: {err}\nConsider restarting RTE.")
        }

        let mut buf = String::new();
        if let Err(_) = io::stdin().read_line(&mut buf) { return Err(String::from("Could not read line from Stdin.")) }
        let index = match line.parse::<usize>() {
            Ok(i) => i,
            Err(_) => return Err(String::from("Error while converting index to number!"))
        };

        if buf.trim().is_empty() { return Ok(()) }

        let text = match buf.strip_suffix("\n") {
            None => buf,
            Some(t) => t.to_string()
        };

        self.lines.insert(index, text);
        self.lines.remove(index - 1);
        Ok(())
    }

    fn delete_line(&mut self, line: usize) -> Result<(), String> {
        if line > self.lines.len() { return Err("Invalid Index.".to_string()) }
        self.lines.remove(line - 1);
        Ok(())
    }

    fn insert_line(&mut self, line: usize) -> Result<(), String>{
        if line > self.lines.len() { return Err("Invalid Index.".to_string()) }

        println!("Enter text you want to insert at index {} and press Enter to confirm or leave blank to cancel.", line);
        print!("{} | ", line);
        if let Err(err) = stdout().flush() { return Err(err.to_string()) }

        let mut buf = String::new();
        if let Err(err) = io::stdin().read_line(&mut buf) { return Err(err.to_string()) }
        
        let text = match buf.strip_suffix("\n") {
            None => buf,
            Some(t) => t.to_string()
        };

        if text.is_empty() { return Ok(()) }
        self.lines.insert(line - 1, text);

        Ok(())
    }

    fn append_lines(&mut self) -> io::Result<()> {
        println!("Enter text. A single '.' on a line by itself ends append mode.");
        let mut line = 1;

        loop {
            print!("{} | ", line);
            io::stdout().flush()?;

            let mut buf = String::new();
            let n = io::stdin().read_line(&mut buf)?;
            if n == 0 {
                break
            }

            let trimmed = buf.trim_end_matches(&['\r', '\n'][..]);
            if trimmed == "." {
                break
            }

            self.lines.push(trimmed.to_string());
            self.dirty = true;
            line += 1
        }

        Ok(())
    }

    fn write_file(&mut self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&self.filename)?;

        for line in &self.lines {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?
        }

        file.flush()?;
        self.dirty = false;
        println!("Wrote {} line/s to {}", self.lines.len(), self.filename.display());
        Ok(())
    }
}

pub fn rte(args: &[String]) -> Result<(), String> {
    if let None = args.get(0) { return Err("Usage: rte <Target File>".to_string()) }

    let filename = PathBuf::from(args[0].trim());
    let mut editor = match Editor::new(filename) {
        Ok(ed) => ed,
        Err(err) => { 
            eprintln!("Error: {}", err); 
            std::process::exit(1) 
        }
    };

    println!("Opened {} ({} line/s)", editor.filename.display(), editor.lines.len());
    println!("Type 'h' for help.");
    let stdin = io::stdin();

    loop {
        print!("RTE >> ");
        if let Err(err) = io::stdout().flush() {
            println!("Important: flushing stdout failed!\nError: {}\nConsider restarting RTE.\n", err)
        }

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(lines) => lines,
            Err(err) => { 
                println!("Important: Reading from stdin failed! Error: {}\nConsider restarting RTE.\n", err); 
                continue 
            }
        };

        if input.is_empty() { continue }
        let mut parts = input.trim().split_whitespace();
        let cmd = match parts.next() {
            Some(c) => c,
            None => continue
        };

        match cmd {
            "r" => {
                let line_num = parts.next().and_then(|s| s.parse::<usize>().ok());
                if let Err(err) = editor.read_lines(line_num) {
                    eprintln!("Error: {}", err)
                }
            }

            "a" => {
                if let Err(e) = editor.append_lines() {
                    eprintln!("Error: {e}")
                }
            }

            "e" => {
                let index = match parts.next() {
                    Some(i) => i,
                    None => { eprintln!("Expected value after 'e'."); continue },
                };

                if let Err(err) = editor.edit_line(index) { eprintln!("{err}"); continue }
            }

            "d" => {
                let index = match parts.next() {
                    Some(i) => i,
                    None => { eprintln!("Expected value after 'e'."); continue },
                };

                match index.parse::<usize>() {
                    Err(_) => { eprintln!("Could not parse index to a number."); continue }
                    Ok(i) => { 
                        if let Err(err) = editor.delete_line(i) { eprintln!("Error: {err}") }
                    }
                }
            }

            "i" => {
                let index = match parts.next() {
                    Some(i) => i,
                    None => { eprintln!("Expected value after 'i'."); continue }
                };

                match index.parse::<usize>() {
                    Err(_) => { eprintln!("Could not parse index to a number."); continue }
                    Ok(i) => { 
                        if let Err(err) = editor.insert_line(i) { eprintln!("Error: {err}") }
                    }
                }
            }

            "w" => { 
                if let Err(err) = editor.write_file() {
                    eprintln!("Error: {err}")
                }
            }

            "q" => {
                if editor.dirty {
                    eprintln!("Buffer modified; use 'w' to save or 'q!' to quit without saving")
                } else {
                    return Ok(())
                }
            }

            "q!" => return Ok(()),
            "h" => print_help(),
            _ => eprintln!("Unknown command '{}'; try 'h'", cmd)
        }
    }
}
