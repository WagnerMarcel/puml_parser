use puml_parser::parser::diagram::Diagram;

mod common;

#[test]
fn system_test() {
    let mut diagram = Diagram::new();
    let theme = "";
    diagram.create("tests/test_class.cpp", theme);
    assert_eq!(
        diagram.file_contents.join("\n"),
        common::get_expected_diagram()
    );
}
