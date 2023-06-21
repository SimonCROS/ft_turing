pub mod machine_parser;
pub mod machine_runner;
use std::fs;
use std::env;

fn print_help() {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        if args[1] == "--help" || args[1] == "-h" {
            print_help();
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
                    let input = args[2].to_owned();
                    let exec = machine_runner::machine_start(&m, input);
                    match exec {
                        Ok(ribbon) => {
                            println!("Machine [{}] has run successfully !", m.name);
                            println!("End ribbon is:");
                            println!("[{}]", ribbon);
                        },
                        Err(error) => { println!("{}", error); return ();},
                    };
                },
                Err(error) => { println!("JSON LOGIC ERROR: {}", error); return ();},
            };
        },
        Err(error) => eprintln!("JSON SYNTAX ERROR: {}", error),
    }
}
