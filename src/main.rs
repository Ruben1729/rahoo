use rahoo::{search};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a function signature as an argument.");
        return;
    }

    let res = search("/run/media/rubens/ssd/projects/calyx-rust/editor/src/", args[1].as_str());
    println!("Found {} results", res.len());
    for node in res {
        println!("{}", node);
    }
}