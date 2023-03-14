use puml_parser::parser::diagram::Diagram;

mod common;

#[test]
fn system_test() {
    let mut diagram = Diagram::new();
    diagram.create("tests/test.cpp");
    assert_eq!(
        diagram.file_contents.join("\n"),
        common::get_expected_diagram()
    );
}
