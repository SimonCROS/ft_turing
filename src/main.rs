pub mod machine_parser;
use std::fs;

fn main() {

    let file_path = "machines/is_palindrome.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    let machine = machine_parser::machine_parser(&contents);

    match machine {
        Err(error) => print!("{}", error),
        Ok(m) => {
            println!("json syntax OK");
            machine_parser::machine_printer(&m);
            let checker = machine_parser::machine_checker(&m);
            match checker {
                Some(error) =>{ println!("{}", error); return ();},
                None => {
                    println!("logic check OK");
                },
            }
        },
    }


    println!("test print end")
}
