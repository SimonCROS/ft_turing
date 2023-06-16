use serde::{Deserialize, Serialize};
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

pub fn check_duplicates<T>(list: &Vec<T>) -> Option<T>
    where T: PartialEq + Clone {
    match (1..list.len()).find(|i| list[*i..].contains(&list[i - 1])) {
        Some(duplicated) => Some(list[duplicated].clone()),
        None => None
    }
}

pub fn check_contains_all<T>(needle: &Vec<T>, haystack: &Vec<T>) -> Option<T>
    where T: PartialEq + Clone {
    match needle.iter().find(|el| !haystack.contains(el)) {
        Some(not_in) => Some(not_in.clone()),
        None => None
    }
}

pub fn machine_checker(m: &Machine) -> Result<(), String> {
    // -Your program must detect and reject ill formated or invalid machine descriptions
    // and inputs, with a relevant error message. This means that your program must
    // never crash for any reason.
    
    if m.alphabet.len() < 2 {
        // Error if the alphabet has less than 2 symbols (blank + 1 other minimum)
        return Err(format!("a minimum of 2 alphabet symbol are required"));
    } else if m.states.len() < 2 {
        // Error if there are less than 2 states (initial + 1 final minimum)
        return Err(format!("a minimum of 2 states are required"));
    } else if m.finals.len() < 1 {
        // Error if there is no final state
        return Err(format!("at least one final state is required"));
    } else if !m.alphabet.contains(&m.blank) {
        // Error if the blank character is not part of the alphabet
        return Err(format!("blank symbol [{}] is not part of alphabet", m.blank));
    } else if !m.states.contains(&m.initial) {
        // Error if the initial state is not in the states list
        return Err(format!("initial state [{}] is not part of states", m.initial));
    } else if let Some(not_in) = check_contains_all(&m.finals, &m.states) {
        // Error if the initial state is not in the states list
        return Err(format!("final state [{}] is not part of states", not_in));
    } else if let Some(duplicated) = check_duplicates(&m.alphabet) {
        // Error if there is a duplicated alphabet symbol
        return Err(format!("duplicate alphabet symbol [{}]", duplicated));
    } else if let Some(duplicated) = check_duplicates(&m.states) {
        // Error if there is a duplicated state
        return Err(format!("duplicate state [{}]", duplicated));
    }

    //transition stuff
    // -make sure every state (finals excluded) has transitions
    for item in m.states.iter() {
        match item {
            _ => {
                if m.finals.iter().position(|x| x == item) == None {
                    //state is not final, check if it is in transition
                    if !m.transitions.contains_key(item) {
                        return Err(format!("state [{}] has no transitions.", item));
                    }
                }
            }
        }
    }

    // -make sure every transition block is for a state that exists
    for (key, _value) in &m.transitions {
        if m.states.iter().position(|x| x == key) == None {
            return Err(format!("transition [{}] is not part of states", key));
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
                    return Err(format!("in transition [{}] there is multiple statement for read [{}]", key, value[i].read));
                }
            }

            //check if final is present
            if !found_final && m.finals.iter().position(|x| x == &value[i].to_state) != None {
                found_final = true;
            }

            // -action can only be RIGHT or LEFT
            if !(value[i].action == "LEFT" || &value[i].action == "RIGHT") {
                return Err(format!("statement action [{}] from [{}] read [{}] should be [RIGHT] or [LEFT] only", value[i].action, key, value[i].read));
            }

            // -make sure "to_state" statements use only stuff from "states"
            if m.states.iter().position(|x| x == &value[i].to_state) == None {
                return Err(format!("statement to_state [{}] from [{}] read [{}] is not part of states", value[i].to_state, key, value[i].read));
            }

            // -make sure read statements use only stuff from "alphabet"
            if m.alphabet.iter().position(|x| x == &value[i].read) == None {
                return Err(format!("statement read [{}] from [{}] read [{}] is not part of alphabet", value[i].read, key, value[i].read));
            }
        }
    }

    // -at least one HALT statement must be present in a to_state statement
    if !found_final {
        return Err(format!("at least one final state must be present in the to_state statements"));
    }

    Ok(())
}
