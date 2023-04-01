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
        "@enduml",
    ]
    .join("\n")
}
