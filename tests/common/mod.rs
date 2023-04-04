pub fn get_expected_diagram() -> String {
    [
        "@startuml",
        "class TestClass {",
        "+m_enumValue : TestClass::TestEnum",
        "+get()",
        "+m_myInt : unsigned int",
        "#m_structValue : TestClass::TestStruct",
        "#m_protectedInt : int",
        "-someFunc()",
        "-m_someVar : float",
        "}",
        "struct TestStruct {",
        "+one : int",
        "}",
        "TestClass o-- TestStruct",
        "@enduml",
    ]
    .join("\n")
}
