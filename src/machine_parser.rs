use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

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

pub fn machine_checker(m: &Machine) -> Option<String> {
    // -Your program must detect and reject ill formated or invalid machine descriptions
    // and inputs, with a relevant error message. This means that your program must
    // never crash for any reason.

    //check alphabet has no duplicate
    for i in 0..(m.alphabet.len() - 1) {
        for j in (i + 1)..(m.alphabet.len()) {
            if m.alphabet[i] == m.alphabet[j] {
                return Some(format!("duplicate alphabet symbol [{}]", m.alphabet[i]));
            }
        }
    }

    // -blank: The blank character, must be part of the alphabet
    if m.alphabet.iter().position(|x| x == &m.blank) == None {
        return Some(format!(
            "blank symbol [{}] is not part of alphabet",
            m.blank
        ));
    }

    //states cannot be duplicate
    for i in 0..(m.states.len() - 1) {
        for j in (i + 1)..(m.states.len()) {
            if m.states[i] == m.states[j] {
                return Some(format!("duplicate state [{}]", m.states[i]));
            }
        }
    }

    // (more parsing check stuff)
    // -initial and finals states must be included in states

    //initial
    if m.states.iter().position(|x| x == &m.initial) == None {
        return Some(format!(
            "initial state [{}] is not part of states",
            m.initial
        ));
    }

    //finals
    for item in m.finals.iter() {
        match item {
            _ => {
                if m.states.iter().position(|x| x == item) == None {
                    return Some(format!("final state [{}] is not part of states", item));
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
                        return Some(format!("state [{}] has no transitions.", item));
                    }
                }
            }
        }
    }

    // -make sure every transition block is for a state that exists
    for (key, _value) in &m.transitions {
        if m.states.iter().position(|x| x == key) == None {
            return Some(format!("transition [{}] is not part of states", key));
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
                    return Some(format!(
                        "in transition [{}] there is multiple statement for read [{}]",
                        key, value[i].read
                    ));
                }
            }

            //check if final is present
            if !found_final && m.finals.iter().position(|x| x == &value[i].to_state) != None {
                found_final = true;
            }

            // -action can only be RIGHT or LEFT
            if !(value[i].action == "LEFT" || &value[i].action == "RIGHT") {
                return Some(format!(
                    "statement action [{}] from [{}] read [{}] should be [RIGHT] or [LEFT] only",
                    value[i].action, key, value[i].read
                ));
            }

            // -make sure "to_state" statements use only stuff from "states"
            if m.states.iter().position(|x| x == &value[i].to_state) == None {
                return Some(format!(
                    "statement to_state [{}] from [{}] read [{}] is not part of states",
                    value[i].to_state, key, value[i].read
                ));
            }

            // -make sure read/write statements use only stuff from "alphabet"
            //read
            if m.alphabet.iter().position(|x| x == &value[i].read) == None {
                return Some(format!(
                    "statement read [{}] from [{}] read [{}] is not part of alphabet",
                    value[i].read, key, value[i].read
                ));
            }

            //write
            // if m.alphabet.iter().position(|x| x == &value[i].write) == None
            // {
            //     return Some(format!("statement wirte [{}] from [{}] read [{}] is not part of alphabet", value[i].write, key, value[i].read));
            // }
        } //end for item in value.iter()
    }

    // -at least one HALT statement must be present in a to_state statement
    if !found_final {
        return Some(format!(
            "at least one final state must be present in the to_state statements"
        ));
    }

    return None;
}
