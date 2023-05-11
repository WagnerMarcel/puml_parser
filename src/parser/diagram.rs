use clang::diagnostic::Diagnostic;
use clang::*;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

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
        let path = Path::new(file);
        self.file_contents.push(String::from("@startuml"));
        if !theme.is_empty() {
            self.file_contents.push(format!("!theme {}", theme))
        }

        self.file_contents.push(String::from("hide empty members"));

        // Acquire an instance of `Clang`
        let clang = Clang::new().unwrap();

        // Create a new `Index`
        let index = Index::new(&clang, false, true);

        // Parse a source file into a translation unit using Cpp11
        let result = index.parser(file).arguments(&["-std=c++11"]).parse();

        println!("Parsing finished!");
        let mut tu;
        match result {
            Ok(_tu) => tu = _tu,
            Err(err) => {
                println!("{:?}", err);
                return;
            }
        }

        let diag = tu.get_diagnostics();

        if !diag.is_empty() {
            println!("Unresolved diagnostics, prepending unknown types into temp file.");
            let content = Self::handle_unknown_types(file, tu.get_diagnostics());

            let file_name = format!(
                "{}/t_{}",
                path.parent().unwrap().to_str().unwrap(),
                path.file_name().unwrap().to_str().unwrap()
            );

            let mut f;
            let result = fs::File::create(&file_name);
            match result {
                Ok(_f) => f = _f,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            }

            match f.write_all(content.as_slice()) {
                Ok(_) => println!("Successfully created temp file."),
                Err(err) => println!("{}", err),
            }

            tu = index
                .parser(&file_name)
                .arguments(&["-std=c++11"])
                .parse()
                .unwrap();

            match fs::remove_file(&file_name) {
                Ok(_) => println!("Successfully removed temp file."),
                Err(err) => println!("{}", err),
            }
        }

        let entities = tu.get_entity().get_children();

        for entity in entities {
            let mut element = Element::default();
            element.create(entity);
            let mut vec = element.get();
            self.file_contents.append(&mut vec);
        }

        self.file_contents.push(String::from("@enduml"));
    }

    pub fn write_to_file(self, f: &str) {
        match fs::write(f, self.get_diagram()) {
            Ok(_) => log!("Save successful"),
            Err(error) => println!("{}", error),
        }
    }

    fn handle_unknown_types(file: &str, diagnostics: Vec<Diagnostic>) -> Vec<u8> {
        let mut content: Vec<u8> = vec![];
        for diag in diagnostics {
            let diag_text = diag.get_text();
            let search_string = String::from("unknown type name");
            if diag_text.contains(&search_string) {
                let stripped = diag_text.strip_prefix(&search_string).unwrap().to_string();

                let trimmed = stripped.trim().trim_matches('\'');

                warn_!("unknown type", trimmed);

                let missing_type_definition = format!("struct {} {{}};\n\n", trimmed);
                let data = missing_type_definition.as_bytes();

                content = data.to_owned();
            }
        }

        let mut f = fs::File::open(file).unwrap();

        let mut buffer: Vec<u8> = vec![];
        f.read_to_end(&mut buffer).unwrap();

        content.append(&mut buffer);
        content
    }
}
