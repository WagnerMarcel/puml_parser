pub fn get_expected_diagram() -> String {
    "@startuml\nclass D {\n-a\n-b\n-c\n-d\n}\nclass E {\n+get()\n+m_myInt\n#m_protectedInt\n-someFunc()\n-m_someVar\n}\nclass F {\n-a\n-b\n-c\n-d\n}\n@enduml".to_string()
}
