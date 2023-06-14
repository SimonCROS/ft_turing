pub mod machine_parser;
use std::fs;

fn main() {

    // let result = machine_parser::typed_example();


    let file_path = "machines/is_0n1n.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    // parser::untyped_example(&contents);
    let result = machine_parser::typed_example(&contents);

    match result {
        Ok(_) => (),
        Err(error) => print!("{}", error),
    }
}

// println!("Hello, world!");

//     // Grab JSON file
//     let file_path = "data/test.json".to_owned();
//     let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

//     parser::untyped_example(&contents);