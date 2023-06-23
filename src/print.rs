use crate::machine_parser::Machine;

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

pub fn print_machine(m: &Machine) {
    println!("{:^60}", "machine name:");
    println!("{:^60}", m.name);
    print!("{:<10}", "alphabet:");
    for item in m.alphabet.iter() {
        match item {
            _ => print!("[{}] ", item),
        }
    }
    println!("");
    print!("{:<10}", "blank:");
    println!("[{}]", m.blank);
    print!("{:<10}", "states: ");
    for item in m.states.iter() {
        match item {
            _ => print!("[{}] ", item),
        }
    }
    println!("");
    print!("{:<10}", "initial:");
    println!("[{}]", m.initial);
    print!("{:<10}", "finals: ");
    for item in m.finals.iter() {
        match item {
            _ => print!("[{}] ", item),
        }
    }
    println!("");
    println!("{:^60}", "transitions:");
    println!("{:^12 }|{: ^24}|{: ^12}|{: ^12}", "read", "to_state", "write", "action");
    for (key, value) in m.transitions.iter() {
        println!("{:#^60}", key);
        for item in value.iter() {
            match item {
                _ => println!("{: ^12}|{: ^24}|{: ^12}|{: ^12}", item.read, item.to_state, item.write, item.action),
            }
        }
    }
}
