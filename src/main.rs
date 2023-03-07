use std::fs;

use regex::Regex;

// Final design should look something like this:
// 1. Split file into (nested) blocks.
//    Each block is set by curly braces with a name and a type infront
// 2. Detetmine if block should be private, protected or public.
// 3. Process each block (maybe recusively)
//    - Determine type of block
//      - class is indicated by special type
//      - function is defined by round braces
//      - rest should be variables

fn split_keep<'a>(r: &Vec<&str>, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for pat in r {
        for (index, matched) in text.match_indices(pat) {
            if last != index {
                result.push(&text[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

struct Class {
    m_type: String,
    m_name: String,
    m_block: String,
}

impl Class {
    pub fn new(text: &str) -> Self {
        let regex = Regex::new(r"(\S*) (\S*).*\{([\s\S]*)\}").unwrap();

        let capture = regex
            .captures(&text)
            .expect("Regex for parsing class failed.");

        Self {
            m_type: capture[1].trim().to_string(),
            m_name: capture[2].trim().to_string(),
            m_block: capture[3].to_string(),
        }
    }

    pub fn print(self: &Self) {
        println!(
            "Type:\t{}\nName:\t{}\nBlock: {}",
            &self.m_type, &self.m_name, &self.m_block
        );
    }

    pub fn to_puml(self: &Self) -> String {
        let mut puml_diagram = vec!["@startuml"];

        let puml_class = format!("{} {} {{\n", self.m_type, self.m_name).to_owned();
        puml_diagram.push(&puml_class);
        puml_diagram.push("}\n@enduml");
        puml_diagram.join("\n").to_string()
    }
}

fn main() {
    let file_contents = fs::read_to_string("test.cpp").expect("Read string failed");

    if file_contents.contains("class") {
        let parsed_class: Class = Class::new(&file_contents);

        parsed_class.print();

        let puml = parsed_class.to_puml();
        println!("{}", puml);
        fs::write("test.puml", puml).expect("Write failed.");
    }

    // if re_type == &"class" {
    //     let visibility_capture =
    //         split_keep(&["public:", "protected:", "private:"].to_vec(), re_block);

    //     for find in visibility_capture {
    //         println!("{find}");
    //     }
    // }
}
