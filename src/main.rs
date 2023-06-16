pub mod machine_parser;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        if args[1] == "--help" || args[1] == "-h" {
            println!("usage: ft_turing [-h] jsonfile input");
            println!();
            println!("positional arguments:");
            println!("  {:<20} json description of the machine", "jsonfile");
            println!();
            println!("  {:<20} input of the machine", "input");
            println!();
            println!("optional arguments:");
            println!("  {:<20} show this help message and exit", "-h, --help");
            println!();
        }
        else {
            println!("Wrong number of arguments");
            println!("usage: ft_turing [-h] jsonfile input");
        }
        return;
    }
    
    let file_path = args[1].to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let machine = serde_json::from_str(&contents);

    match machine {
        Ok(m) => {
            let checker = machine_parser::machine_checker(&m);
            match checker {
                Ok(()) => {
                    machine_parser::machine_printer(&m);
                },
                Err(error) => { println!("{}", error); return ();},
            };
            
            //here 
            println!("popopopo");
            // let input = args[2];


        },
        Err(error) => eprintln!("{}", error),
    }
}
