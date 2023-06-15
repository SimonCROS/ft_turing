pub mod machine_parser;
use std::fs;

fn main() {

    let file_path = "machines/is_0n1n.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    let machine = machine_parser::machine_parser(&contents);

    match machine {
        Err(error) => print!("{}", error),
        Ok(m) => machine_parser::machine_printer(m),
    }

    println!("test print")
}
