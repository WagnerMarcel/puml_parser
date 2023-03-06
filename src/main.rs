use std::fs;

// Final design should look something like this:
// 1. Split file into (nested) blocks. 
//    Each block is set by curly braces with a name and a type infront
// 2. Detetmine if block should be private, protected or public.
// 3. Process each block (maybe recusively)
//    - Determine type of block
//      - class is indicated by special type
//      - function is defined by round braces
//      - rest should be variables

fn main() {
    let contents = fs::read_to_string("test.cpp").expect("Read string failed");

    // Test for getting the class name (does not follow the above described block structure).
    for line in contents.split_terminator("\n") {
        if line.contains("class") {
            let class_name = line
                .strip_prefix("class")
                .expect("class didnt contain class")
                .strip_suffix("{")
                .expect("Class name didnt contain opening brace")
                .trim();
            println!("{class_name}");
        }
    }
}
