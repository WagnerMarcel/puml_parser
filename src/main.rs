use clang::*;

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
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, true);

    // Parse a source file into a translation unit
    let tu = index
        .parser("test.cpp")
        .arguments(&["-std=c++11"])
        .parse()
        .unwrap();

    let entities = tu.get_entity().get_children().into_iter();

    for entity in entities {
        println!("{:?}", entity.get_pretty_printer());
    }

    // Get the structs in this translation unit
    let structs = tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::StructDecl)
        .collect::<Vec<_>>();

    let classes = tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::ClassDecl)
        .collect::<Vec<_>>();

    // Print information about the structs
    for struct_ in structs {
        let type_ = struct_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        println!(
            "struct: {:?} (size: {} bytes)",
            struct_.get_name().unwrap(),
            size
        );

        println!("Number of children: {}", struct_.get_children().len());

        for field in struct_.get_children() {
            let name = field.get_name().expect("Name failed");
            let offset = type_.get_offsetof(&name).unwrap();
            println!("    field: {:?} (offset: {} bits)", name, offset);
        }

        println!("{:?}", struct_.get_pretty_printer());
    }

    for class_ in classes {
        let type_ = class_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        println!(
            "class: {:?} (size: {} bytes)",
            class_.get_name().unwrap(),
            size
        );

        println!("Number of children: {}", class_.get_children().len());

        for field in class_.get_children() {
            println!("{:?}", field.get_pretty_printer());
        }

        println!("{:?}", class_.get_pretty_printer());
    }
}
