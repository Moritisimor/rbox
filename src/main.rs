mod file;
mod dir;
pub mod internals;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { println!("Usage: rbox <command> <options>"); return }

    match args[1].trim() {
        "crtf" => file::crtf(args),
        "rdf" => file::rdf(args),
        "crtd" => dir::crtd(args),
        "ls" => dir::ls(args),
        _ => println!("Unknown command!")
    }
}
