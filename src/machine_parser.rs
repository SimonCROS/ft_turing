use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Transition {
    read: String,
    to_state: String,
    write: String,
    action: String
}

#[derive(Serialize, Deserialize)]
pub struct Machine {
    name: String,
    alphabet: Vec<String>,
    blank: String,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transition>>
}

pub fn machine_parser(data: &str) -> Result<Machine> {

    let m: Machine = serde_json::from_str(data)?;
    return Ok(m)
}


pub fn machine_printer(m: Machine) -> () {

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
    print!("{:<10}","finals: ");
    for item in m.finals.iter() {
        match item {
            _ => print!("[{}] ", item),
        }
    }
    println!("");
    println!("{:^60}", "transitions:");
    println!("{:^12 }|{: ^24}|{: ^12}|{: ^12}",
    "read", "to_state", "write", "action");
    for (key, value) in m.transitions.iter()  {
        println!("{:#^60}", key);
        for item in value.iter() {
            match item {
                _ => println!("{: ^12}|{: ^24}|{: ^12}|{: ^12}",
                            item.read, item.to_state, item.write, item.action
                ),
            }
        }
    }
}