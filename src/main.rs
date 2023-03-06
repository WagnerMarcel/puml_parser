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

fn main() {
    let contents = fs::read_to_string("test.cpp").expect("Read string failed");

    let class_regex = Regex::new(r"(\S*) (\S*).*\{([\s\S]*)\}").unwrap();

    let class_capture = class_regex.captures(&contents).expect("Regex failed.");

    let (re_type, re_name, re_block) = (
        &class_capture[1].trim(),
        &class_capture[2].trim(),
        &class_capture[3],
    );

    println!("Type: {} Name: {} Block: {}", &re_type, &re_name, &re_block);

    if re_type == &"class" {
        let visibility_capture =
            split_keep(&["public:", "protected:", "private:"].to_vec(), re_block);

        for find in visibility_capture {
            println!("{find}");
        }
    }
}
