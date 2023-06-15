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


pub fn machine_printer(m: &Machine) -> () {

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

pub fn machine_checker(m: &Machine) -> Option<&'static str> {


    // -Your program must detect and reject ill formated or invalid machine descriptions
    // and inputs, with a relevant error message. This means that your program must
    // never crash for any reason.




    //check alphabet contains only char
    for item in m.alphabet.iter() {
        match item {
            _ => {
                // println!("checking item:{}", item);
                if item.len() != 1
                {
                    println!("Alphabet [{}] is not a char", item);
                    return Some("Machine logic check error");
                }
                //else{println!("{} is a char", item);}
            },
        }
    }


    //check alphabet has no duplicate
    for i in 0..(m.alphabet.len() - 1)
    {
        for j in (i+1)..(m.alphabet.len())
        {
             if m.alphabet[i] == m.alphabet[j]
            {
                println!("duplicate alphabet symbol [{}]", m.alphabet[i]);
                return Some("Machine logic check error");
            }
        }
    }

    // -blank: The blank character, must be part of the alphabet
    




    //states cannot be duplicate
    for i in 0..(m.states.len() - 1)
    {
        for j in (i+1)..(m.states.len())
        {
             if m.states[i] == m.states[j]
            {
                println!("duplicate state [{}]", m.states[i]);
                return Some("Machine logic check error");
            }
        }
    }
    
    // (more parsing check stuff)
    // -initial and finals states must be included in states
    
    
    //transition stuff
    // -make sure every state (finals excluded) has transitions
    // -cannot have 2 read statement for the same char in the same state
    // -at least one HALT statement must be present in a to_state statement
    // -action can only be RIGHT or LEFT
    // -make sure read/write statements use only stuff from "alphabet"
    // -make sure "to_state" statements use only stuff from "states"



   return None;
}