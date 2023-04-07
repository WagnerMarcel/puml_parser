pub fn get_expected_diagram() -> String {
    [
        "@startuml",
        "hide empty members",
        "class TestClass {",
        "+m_enumValue : TestClass::TestEnum",
        "+get()",
        "+m_myInt : unsigned int",
        "#m_structValue : TestClass::TestStruct",
        "#m_protectedInt : int",
        "-someFunc()",
        "-m_someVar : float",
        "}",
        "enum TestEnum {",
        "One",
        "Two",
        "}",
        "TestClass o-- TestEnum",
        "class TestStruct {",
        "+one : int",
        "}",
        "TestClass o-- TestStruct",
        "@enduml",
    ]
    .join("\n")
}
