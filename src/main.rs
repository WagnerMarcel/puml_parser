use std::process::Command;

mod parser;
use parser::diagram::Diagram;

pub mod utils;

use std::env;

// Final design should look something like this:
// 1. Split file into (nested) blocks.
//    Each block is set by curly braces with a name and a type infront
// 2. Detetmine if block should be private, protected or public.
// 3. Process each block (maybe recusively)
//    - Determine type of block
//      - class is indicated by special type
//      - function is defined by round braces
//      - rest should be variables

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let save_path = file[0..file.chars().position(|c| c == '.').unwrap()].to_string() + ".puml";
    println!("{}", save_path);

    let mut theme = "";
    if args.len() > 2 {
        theme = &args[2];
        println!("{}", theme);
    }

    let mut diagram = Diagram::new();
    diagram.create(&file, &theme);
    diagram.write_to_file(&save_path);

    Command::new("sh")
        .arg("-c")
        .arg("plantuml -tsvg ".to_string() + save_path.as_str())
        .output()
        .expect("failed to execute");
}
