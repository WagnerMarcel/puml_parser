use clang::*;
use std::fs;

use super::super::utils::macros::*;

pub struct Diagram {
    pub file_contents: Vec<String>,
}

impl Default for Diagram {
    fn default() -> Self {
        Self::new()
    }
}

impl Diagram {
    pub fn new() -> Self {
        Self {
            file_contents: (vec![]),
        }
    }

    pub fn create(&mut self, file: &str, theme: &str) {
        self.file_contents.push("@startuml".to_string());
        if !theme.is_empty() {
            self.file_contents.push(format!("!theme {}", theme))
        }

        // Acquire an instance of `Clang`
        let clang = Clang::new().unwrap();

        // Create a new `Index`
        let index = Index::new(&clang, false, true);

        // Parse a source file into a translation unit using Cpp11
        let tu = index
            .parser(file)
            .arguments(&["-std=c++11"])
            .parse()
            .unwrap();

        let entities = tu.get_entity().get_children();

        for entity in entities {
            match entity.get_kind() {
                EntityKind::ClassDecl => {
                    self.create_class(&entity);
                }
                kind => warn_unimplemented!(format!("{:?}", kind)),
            }
        }

        self.file_contents.push("@enduml".to_string());
    }

    fn create_class(&mut self, class_entity: &Entity) {
        if let Some(name) = class_entity.get_name() {
            self.file_contents.push(format!("class {} {{", name));
        }

        for field in class_entity.get_children() {
            match field.get_kind() {
                EntityKind::FieldDecl => self.create_declaration(&field),
                EntityKind::Method => self.create_method(&field),
                kind => warn_unimplemented!(format!("{:?}", kind)),
            }
        }

        self.file_contents.push("}".to_string());
    }

    fn create_declaration(&mut self, field_entity: &Entity) {
        println!("{:?}", field_entity.get_type().unwrap());
        match Diagram::get_accessibility_character(field_entity) {
            Some(accessibility) => self.file_contents.push(format!(
                "{}{} : {}",
                accessibility,
                field_entity.get_name().unwrap_or_default(),
                field_entity.get_type().unwrap().get_display_name()
            )),
            None => self
                .file_contents
                .push(field_entity.get_name().unwrap_or_default()),
        }
    }

    fn create_method(&mut self, method_entity: &Entity) {
        match Diagram::get_accessibility_character(method_entity) {
            Some(accessibility) => self.file_contents.push(format!(
                "{}{}()",
                accessibility,
                method_entity.get_name().unwrap_or_default()
            )),
            None => self.file_contents.push(format!(
                "{}()",
                method_entity.get_name().unwrap_or_default()
            )),
        }
    }

    fn get_accessibility_character(entity: &Entity) -> Option<char> {
        match entity.get_accessibility() {
            Some(Accessibility::Private) => Some('-'),
            Some(Accessibility::Protected) => Some('#'),
            Some(Accessibility::Public) => Some('+'),
            None => None,
        }
    }

    pub fn write_to_file(self, f: &str) {
        match fs::write(f, self.file_contents.join("\n")) {
            Ok(_) => log!("Save successful"),
            Err(error) => println!("{}", error),
        }
    }
}
