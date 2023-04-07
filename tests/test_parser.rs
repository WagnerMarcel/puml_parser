use puml_parser::parser::diagram::Diagram;
use serial_test::serial;

mod common;

#[test]
#[serial]
fn system_test_cpp() {
    let mut diagram = Diagram::new();
    let theme = "";
    diagram.create("tests/test_class.cpp", theme);
    assert_eq!(diagram.get_diagram(), common::get_expected_diagram(None));
}

#[test]
#[serial]
fn system_test_hpp() {
    let mut diagram = Diagram::new();
    let theme = "";
    diagram.create("tests/test_class.hpp", theme);
    assert_eq!(diagram.get_diagram(), common::get_expected_diagram(None));
}

#[test]
#[serial]
fn namespace_test() {
    let mut diagram = Diagram::new();
    let theme = "";
    diagram.create("tests/test_namespace.hpp", theme);
    assert_eq!(
        diagram.get_diagram(),
        common::get_expected_diagram(Some(true))
    );
}
