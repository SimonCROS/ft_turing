pub mod machine_parser;
use std::fs;

fn main() {

    let file_path = "machines/unary_sub.subject.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    let machine = serde_json::from_str(&contents);

    match machine {
        Ok(m) => {
            println!("json syntax OK");
            machine_parser::machine_printer(&m);
            let checker = machine_parser::machine_checker(&m);
            match checker {
                Some(error) => { println!("{}", error); return ();},
                None => {
                    println!("logic check OK");
                },
            }
        },
        Err(error) => print!("{}", error),
    }


    println!("test print end")
}
