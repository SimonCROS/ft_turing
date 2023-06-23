use crate::machine_parser::{Machine, Transition};

pub fn print_help() {
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

pub fn print_output_header() {
    println!("{:#^80}", "");
    println!("{:#^80}", "");
    println!("{:^15}|{:^23}|{:^3}|{:^23}|{:^3}|{:^8}", "Ribbon", "Current State", "R", "New State", "W", "action");
}

fn print_transitions(name: &String, transitions: &Vec<Transition>) {
    println!("{:#^60}", name);
    transitions.iter().for_each(|item| println!("{: ^12}|{: ^24}|{: ^12}|{: ^12}", item.read, item.to_state, item.write, item.action));
}

pub fn print_machine(m: &Machine) {
    println!("{:^60}", "machine name:");
    println!("{:^60}", m.name);
    println!("{:<10}{}", "alphabet: ", m.alphabet.iter().fold(String::new(), |acc, item| format!("{} [{}]", acc, item)));
    println!("{:<10} [{}]", "blank:", m.blank);
    println!("{:<10}{}", "states: ", m.states.iter().fold(String::new(), |acc, item| format!("{} [{}]", acc, item)));
    println!("{:<10} [{}]", "initial:", m.initial);
    println!("{:<10}{}", "finals: ", m.finals.iter().fold(String::new(), |acc, item| format!("{} [{}]", acc, item)));
    println!("{:^60}", "transitions:");
    println!("{:^12 }|{: ^24}|{: ^12}|{: ^12}", "read", "to_state", "write", "action");
    m.transitions.iter().for_each(|(k, v)| print_transitions(k, v));
}
