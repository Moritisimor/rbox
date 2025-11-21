mod file;
mod dir;
mod env;
mod mcd;
mod rte;
mod rbp;
pub mod internals;

/* 
The main function, basically a dispatcher which checks the first argument and calls 
the first matching function, and, if necessary, passes on the arguments it got to
that function.
*/
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { println!("Usage: rbox <command> <options...>"); return Ok(()) }

    match args[1].trim() {
        "rbp" => rbp::rbp(&args[2..]),
        "rte" => rte::rte(&args[2..]),
        "whereami" => env::whereami(),
        "crtf" => file::crtf(&args[2..]),
        "rmf" => file::rmf(&args[2..]),
        "rdf" => file::rdf(&args[2..]),
        "mve" => file::mve(&args[2..]),
        "cpy" => file::cpy(&args[2..]),
        "crtd" => dir::crtd(&args[2..]),
        "rmd" => dir::rmd(&args[2..]),
        "ls" => dir::ls(&args[2..]),
        "mcd" => mcd::mcd(),
        "help" => { println!("Visit: https://github.com/Moritisimor/rbox"); Ok(()) }
        _ => Err("Unknown Command".to_string())
    }
}
