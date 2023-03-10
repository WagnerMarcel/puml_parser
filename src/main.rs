use clang::*;
use std::fs;
use std::process::Command;

// Final design should look something like this:
// 1. Split file into (nested) blocks.
//    Each block is set by curly braces with a name and a type infront
// 2. Detetmine if block should be private, protected or public.
// 3. Process each block (maybe recusively)
//    - Determine type of block
//      - class is indicated by special type
//      - function is defined by round braces
//      - rest should be variables

struct Diagram {
    file_contents: Vec<String>,
}

impl Diagram {
    pub fn new() -> Self {
        Self {
            file_contents: (vec!["@startuml".to_string()]),
        }
    }

    pub fn create(&mut self, file: &str) {
        // Acquire an instance of `Clang`
        let clang = Clang::new().unwrap();

        // Create a new `Index`
        let index = Index::new(&clang, false, true);

        // Parse a source file into a translation unit
        let tu = index
            .parser(file)
            .arguments(&["-std=c++11"])
            .parse()
            .unwrap();

        let entities = tu.get_entity().get_children().into_iter();

        for entity in entities {
            if is_class(&entity) {
                self.create_class(&entity);
            }
        }

        self.file_contents.push("@enduml".to_string());
    }

    fn create_class(&mut self, class_entity: &Entity) {
        match class_entity.get_name() {
            Some(name) => self.file_contents.push(format!("class {} {{", name)),
            None => {}
        }

        for field in class_entity.get_children() {
            match field.get_kind() {
                EntityKind::FieldDecl => self.create_declaration(&field),
                EntityKind::Method => self.create_method(&field),
                kind => println!("{:?}", kind),
            }
        }

        self.file_contents.push("}".to_string());
    }

    fn create_declaration(&mut self, field_entity: &Entity) {
        match Diagram::get_accessibility_character(field_entity) {
            Some(accessibility) => self.file_contents.push(format!(
                "{}{}",
                accessibility,
                field_entity.get_name().unwrap_or_default()
            )),
            None => self
                .file_contents
                .push(format!("{}", field_entity.get_name().unwrap_or_default())),
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
            Some(Accessibility::Private) => return Some('-'),
            Some(Accessibility::Protected) => return Some('#'),
            Some(Accessibility::Public) => return Some('+'),
            None => return None,
        }
    }

    pub fn write_to_file(self, f: &str) {
        match fs::write(f, self.file_contents.join("\n")) {
            Ok(_) => println!("Save successful"),
            Err(error) => println!("{}", error),
        }
    }
}

fn is_class(entity: &Entity) -> bool {
    EntityKind::ClassDecl == entity.get_kind()
}

fn main() {
    let mut diagram = Diagram::new();
    diagram.create("test.cpp");
    diagram.write_to_file("test.puml");

    // Command::new("sh")
    //     .arg("-c")
    //     .arg("plantuml -tsvg test.puml")
    //     .output()
    //     .expect("failed to execute");
}
