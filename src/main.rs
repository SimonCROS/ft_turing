mod machine_parser;
mod machine_runner;
mod input_checker;
mod print;
mod utils;
use std::fs;
use std::env;
use print::*;

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
    
    let file_path: String = args[1].clone();
    let contents: String = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    match serde_json::from_str(&contents) {
        Ok(machine) => {
            match machine_parser::machine_checker(&machine) {
                Ok(()) => {
                    print_machine(&machine);
                    let input: String = args[2].clone();

                    match input_checker::check_input(&machine, &input) {
                        Ok(()) => {
                            match machine_runner::machine_start(&machine, &input) {
                                Ok(ribbon) => {
                                    println!("Machine [{}] has run successfully !", machine.name);
                                    println!("Input was:");
                                    println!("[{}]", input);
                                    println!("End ribbon is:");
                                    println!("[{}]", ribbon);
                                },
                                Err(error) => eprintln!("MACHINE ERROR: {}", error),
                            };
                        },
                        Err(error) => eprintln!("INPUT ERROR: {}", error),
                    }
                },
                Err(error) => eprintln!("JSON LOGIC ERROR: {}", error),
            };
        },
        Err(error) => eprintln!("JSON SYNTAX ERROR: {}", error),
    }
}
