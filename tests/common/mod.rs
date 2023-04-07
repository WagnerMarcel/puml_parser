fn get_diagram_content() -> Vec<String> {
    vec![
        String::from("class TestClass {"),
        String::from("+m_enumValue : TestEnum"),
        String::from("+get()"),
        String::from("+m_myInt : unsigned int"),
        String::from("#m_structValue : TestStruct"),
        String::from("#m_protectedInt : int"),
        String::from("-someFunc()"),
        String::from("-m_someVar : float"),
        String::from("}"),
        String::from("enum TestEnum {"),
        String::from("One"),
        String::from("Two"),
        String::from("}"),
        String::from("TestClass o-- TestEnum"),
        String::from("class TestStruct {"),
        String::from("+one : int"),
        String::from("}"),
        String::from("TestClass o-- TestStruct"),
    ]
}

pub fn get_expected_diagram(namespace: Option<bool>) -> String {
    let mut vec = vec![
        String::from("@startuml"),
        String::from("hide empty members"),
    ];

    if namespace.unwrap_or(false) {
        vec.push(String::from("namespace OuterNamespace {"));
        vec.push(String::from("namespace TestNamespace {"));
    }

    let mut content = get_diagram_content();
    vec.append(&mut content);

    if namespace.unwrap_or(false) {
        vec.push(String::from("}"));
        vec.push(String::from("}"));
    }

    vec.push(String::from("@enduml"));
    vec.join("\n")
}
