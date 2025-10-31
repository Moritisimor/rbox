mod file;
mod dir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 { println!("Usage: rbox <command> <options>"); return }

    match args[1].trim() {
        "crtf" => file::crtf(args),
        "crtd" => dir::crtd(args),
        _ => { println!("Unknown command!") }
    }
}
