pub fn get_expected_diagram() -> String {
    "@startuml\nclass E {\n+get()\n+m_myInt\n#m_protectedInt\n-someFunc()\n-m_someVar\n}\n@enduml"
        .to_string()
}
