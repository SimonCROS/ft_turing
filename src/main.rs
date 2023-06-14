pub mod machine_parser;

fn main() {

    let result = machine_parser::typed_example();

    match result {
        Ok(_) => (),
        Err(error) => print!("{}", error),
    }
}