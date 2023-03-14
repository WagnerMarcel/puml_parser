use puml_parser::parser::diagram::Diagram;
use std::process::Command;

#[test]
fn system_test() {
    let mut diagram = Diagram::new();
    diagram.create("test.cpp");
    diagram.write_to_file("test.puml");

    Command::new("sh")
        .arg("-c")
        .arg("plantuml -tsvg test.puml")
        .output()
        .expect("failed to execute");
}
