use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Transition {
    read: char,
    to_state: String,
    write: char,
    action: String,
}

#[derive(Serialize, Deserialize)]
pub struct Machine {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transition>>,
}

pub fn machine_parser(data: &str) -> Result<Machine> {
    let m: Machine = serde_json::from_str(data)?;
    return Ok(m);
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
    print!("{:<10}", "finals: ");
    for item in m.finals.iter() {
        match item {
            _ => print!("[{}] ", item),
        }
    }
    println!("");
    println!("{:^60}", "transitions:");
    println!(
        "{:^12 }|{: ^24}|{: ^12}|{: ^12}",
        "read", "to_state", "write", "action"
    );
    for (key, value) in m.transitions.iter() {
        println!("{:#^60}", key);
        for item in value.iter() {
            match item {
                _ => println!(
                    "{: ^12}|{: ^24}|{: ^12}|{: ^12}",
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

    //check alphabet has no duplicate
    for i in 0..(m.alphabet.len() - 1) {
        for j in (i + 1)..(m.alphabet.len()) {
            if m.alphabet[i] == m.alphabet[j] {
                println!("duplicate alphabet symbol [{}]", m.alphabet[i]);
                return Some("Machine logic check error");
            }
        }
    }

    // -blank: The blank character, must be part of the alphabet
    if m.alphabet.iter().position(|x| x == &m.blank) == None {
        println!("blank symbol [{}] is not part of alphabet", m.blank);
        return Some("Machine logic check error");
    }

    //states cannot be duplicate
    for i in 0..(m.states.len() - 1) {
        for j in (i + 1)..(m.states.len()) {
            if m.states[i] == m.states[j] {
                println!("duplicate state [{}]", m.states[i]);
                return Some("Machine logic check error");
            }
        }
    }

    // (more parsing check stuff)
    // -initial and finals states must be included in states

    //initial
    if m.states.iter().position(|x| x == &m.initial) == None {
        println!("initial state [{}] is not part of states", m.initial);
        return Some("Machine logic check error");
    }

    //finals
    for item in m.finals.iter() {
        match item {
            _ => {
                if m.states.iter().position(|x| x == item) == None {
                    println!("final state [{}] is not part of states", item);
                    return Some("Machine logic check error");
                }
            }
        }
    }

    //transition stuff
    // -make sure every state (finals excluded) has transitions
    for item in m.states.iter() {
        match item {
            _ => {
                if m.finals.iter().position(|x| x == item) == None {
                    //state is not final, check if it is in transition
                    if !m.transitions.contains_key(item) {
                        println!("state [{}] has no transitions.", item);
                        return Some("Machine logic check error");
                    }
                }
            }
        }
    }

    // -make sure every transition block is for a state that exists
    for (key, _value) in &m.transitions {
        if m.states.iter().position(|x| x == key) == None {
            println!("transition [{}] is not part of states", key);
            return Some("Machine logic check error");
        }
    }

    //inside the transitions

    // -at least one HALT statement must be present in a to_state statement
    //(try to check that during other checks)
    let mut found_final = false;

    for (key, value) in &m.transitions {
        // for item in value.iter() {
        for i in 0..(value.len()) {
            //for each transition in HashMap<String, Vec<Transition>>

            //check double read on same symbol
            for j in (i + 1)..(value.len()) {
                if value[i].read == value[j].read {
                    println!(
                        "in transition [{}] there is multiple statement for read [{}]",
                        key, value[i].read
                    );
                    return Some("Machine logic check error");
                }
            }

            //check if final is present
            if !found_final && m.finals.iter().position(|x| x == &value[i].to_state) != None {
                found_final = true;
            }

            // -action can only be RIGHT or LEFT
            if !(value[i].action == "LEFT" || &value[i].action == "RIGHT") {
                // println!("Wrong action in [{}] read [{}] value [{}] should be [RIGHT] or [LEFT] only", key, value[i].read, value[i].action);
                println!(
                    "statement action [{}] from [{}] read [{}] should be [RIGHT] or [LEFT] only",
                    value[i].action, key, value[i].read
                );
                return Some("Machine logic check error");
            }

            // -make sure "to_state" statements use only stuff from "states"
            if m.states.iter().position(|x| x == &value[i].to_state) == None {
                println!(
                    "statement to_state [{}] from [{}] read [{}] is not part of states",
                    value[i].to_state, key, value[i].read
                );
                return Some("Machine logic check error");
            }

            // -make sure read/write statements use only stuff from "alphabet"
            //read
            if m.alphabet.iter().position(|x| x == &value[i].read) == None {
                println!(
                    "statement read [{}] from [{}] read [{}] is not part of alphabet",
                    value[i].read, key, value[i].read
                );
                return Some("Machine logic check error");
            }

            //write
            // if m.alphabet.iter().position(|x| x == &value[i].write) == None
            // {
            //     println!("statement wirte [{}] from [{}] read [{}] is not part of alphabet", value[i].write, key, value[i].read);
            //     return Some("Machine logic check error");
            // }
        } //end for item in value.iter()
    }

    // -at least one HALT statement must be present in a to_state statement
    if !found_final {
        println!("at least one final state must be present in the to_state statements");
        return Some("Machine logic check error");
    }

    return None;
}
