use clang::*;
use std::fs;

use super::element::*;

use super::super::utils::macros::*;

pub struct Diagram {
    file_contents: Vec<String>,
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

    pub fn get_diagram(&self) -> String {
        self.file_contents.join("\n")
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
            let mut element = Element::default();
            element.create(entity);
            let mut vec = element.get();
            self.file_contents.append(&mut vec);
        }

        self.file_contents.push("@enduml".to_string());
    }

    pub fn write_to_file(self, f: &str) {
        match fs::write(f, self.get_diagram()) {
            Ok(_) => log!("Save successful"),
            Err(error) => println!("{}", error),
        }
    }
}
