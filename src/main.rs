use std::process::Command;

mod parser;
use parser::diagram::Diagram;

pub mod utils;

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
    let mut diagram = Diagram::new();
    diagram.create("test.cpp");
    diagram.write_to_file("test.puml");

    Command::new("sh")
        .arg("-c")
        .arg("plantuml -tsvg test.puml")
        .output()
        .expect("failed to execute");
}
