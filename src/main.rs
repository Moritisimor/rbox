mod file;
mod dir;
mod env;
pub mod internals;

/* 
The main function, basically a dispatcher which checks the first argument and calls 
the first matching function, and, if necessary, passes on the arguments it got to
that function.
*/
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { println!("Usage: rbox <command> <options...>"); return }

    match args[1].trim() {
        "whereami" => env::whereami(),
        "crtf" => file::crtf(args),
        "rdf" => file::rdf(args),
        "crtd" => dir::crtd(args),
        "ls" => dir::ls(args),
        "help" => println!("Visit: https://github.com/Moritisimor/rbox"),
        _ => println!("Unknown command!")
    }
}
