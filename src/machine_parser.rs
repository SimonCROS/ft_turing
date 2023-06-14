use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Transition {
    read: String,
    to_state: String,
    write: String,
    action: String
}

#[derive(Serialize, Deserialize)]
struct Machine {
    name: String,
    alphabet: Vec<String>,
    blank: String,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transition>>
}

pub fn typed_example(data: &str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // let data = r#"
    // {
    //     "name" : "unary_add",
    //     "alphabet": [ ".", "+", "1", "=" ],
    //     "blank" : ".",
    //     "states" : [ "scan_right", "add_one", "carriage_return",  "HALT" ],
    //     "initial" : "scan_right",
    //     "finals" : [ "HALT" ],
    //     "transitions" :{
    //         "scan_right": [
    //             { "read" : ".", "to_state": "scan_right", "write": ".", "action": "RIGHT"},
    //             { "read" : "+", "to_state": "scan_right", "write": "+", "action": "RIGHT"},
    //             { "read" : "1", "to_state": "add_one", "write": ".", "action": "RIGHT"},
    //             { "read" : "=", "to_state": "HALT" , "write": "=", "action": "RIGHT" }
    //         ],
    //         "add_one":
    //         [
    //             { "read" : ".", "to_state": "carriage_return", "write": "1", "action": "LEFT"},
    //             { "read" : "+", "to_state": "add_one", "write": "+", "action": "RIGHT"},
    //             { "read" : "1", "to_state": "add_one", "write": "1", "action": "RIGHT"},
    //             { "read" : "=", "to_state": "add_one" , "write": "=", "action": "RIGHT" }
    //         ],
    //         "carriage_return":
    //         [
    //             { "read" : ".", "to_state": "scan_right", "write": ".", "action": "RIGHT"},
    //             { "read" : "+", "to_state": "carriage_return", "write": "+", "action": "LEFT"},
    //             { "read" : "1", "to_state": "carriage_return", "write": "1", "action": "LEFT"},
    //             { "read" : "=", "to_state": "carriage_return", "write": "=", "action": "LEFT" }
    //         ]
    //     }
    // }"#;

    // let v: Value = serde_json::from_str(json_data)?;
    let p: Machine = serde_json::from_str(data)?;


    println!("=====START====");
    println!("name: {}", p.name);
    println!("alphabet: ");
    for item in p.alphabet.iter() {
        match item {
            _ => println!("   {}", item),
        }
    }
    println!("blank: {}", p.blank);
    println!("states: ");
    for item in p.states.iter() {
        match item {
            _ => println!("   {}", item),
        }
    }
    println!("initial: {}", p.initial);
    println!("finals: ");
    for item in p.finals.iter() {
        match item {
            _ => println!("   {}", item),
        }
    }
    println!("transitions: ");
    for (key, value) in p.transitions.iter()  {
        println!("  name: {}", key);
        for item in value.iter() {
            match item {
                _ => println!("     read: {}, to_state: {}, write: {}, action: {}", item.read, item.to_state, item.write, item.action),
            }
        }
    }
    println!("=====DONE=====");
    Ok(())
}
