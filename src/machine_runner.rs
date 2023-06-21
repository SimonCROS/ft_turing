use crate::machine_parser::Machine;
use crate::machine_parser::Transition;
use colored::Colorize;
use tailcall::tailcall;

//return Err(format!("Something went very wrong"));
//are there just in case and should never happen
//as the relevant checks are made in the json checks


fn input_checker(m: &Machine, input: &String)-> Result<(), String> {
    if input.is_empty() {
        return Err(format!("Input is empty"));
    }

    for c in input.chars() { 
        if c == m.blank {
            return Err(format!("blank char [{}] not allowed in input", m.blank));
        }
        else if m.alphabet.iter().position(|x| x == &c) == None {
            return Err(format!("symbol [{}] is not part of alphabet", c));
        }
    }
    Ok(())
}

fn machine_check_transitions(state: &Vec<Transition>, read: char)-> Result<&Transition, ()> {
    for t in state.iter() {
        if t.read == read {
            return Ok(t);
        }
    }
    return Err(());
}


#[tailcall]
fn machine_step(m: &Machine, ribbon: String, index: usize, state: (&String, &Vec<Transition>))-> Result<String, String> {
 
    let c: char = ribbon.chars().nth(index).unwrap(); 


    let get_transition: Result<&Transition, ()> = machine_check_transitions(state.1, c);
    match get_transition {
        Ok(t) => {
            let left = &ribbon[..index];
            let right = &ribbon[index+1..];

            let colored_ribbon: String = format!("{}{}{}", left, c.to_string().red() , right );
            println!("{:^15} |{:^23}R{:^3}>{:^23}W{:^3}|{:^8}", colored_ribbon, state.0, t.read, t.to_state, t.write, t.action);

            if m.finals.iter().position(|x: &String| x == &t.to_state) != None {
                return Ok(format!("{}{}{}", left, t.write, right));
            }

            let get_new_state: Option<(&String, &Vec<Transition>)> = m.transitions.get_key_value(&t.to_state);
            match get_new_state {
                Some(new_state) => {
                    if t.action == "LEFT" {
                        if index == 0 {
                            let new_ribbon: String = format!("{}{}{}{}",m.blank, left, t.write, right);
                            return machine_step(m, new_ribbon, 0, new_state);
                        }
                        else {
                            let new_ribbon: String = format!("{}{}{}", left, t.write, right);
                            return machine_step(m, new_ribbon, index - 1, new_state);
                        }
                    }
                    else if t.action == "RIGHT" {
                        let new_index: usize = index + 1;
                        if new_index == ribbon.len() {
                            let new_ribbon: String = format!("{}{}{}{}", left, t.write, right, &m.blank);
                            return machine_step(m, new_ribbon, new_index, new_state);
                        }   
                        else {
                            let new_ribbon: String = format!("{}{}{}", left, t.write, right);
                            return machine_step(m, new_ribbon, new_index, new_state);
                        }    
                    }
                    else { return Err(format!("Something went very wrong"));} 
                },
                None => { return Err(format!("Something went very wrong"));},
            };
        },
        Err(()) => { return Err(format!("State [{}] has not transition for read [{}], Aborting", state.0, ribbon.chars().nth(index).unwrap()));},
    };
}

pub fn machine_start(m: &Machine, input: &String)-> Result<String, String> {

    let input_test: Result<(), String> = input_checker(&m, &input);
    match input_test {
        Ok(()) => {},
        Err(error) => { return Err(format!("INPUT ERROR: {}", error));},
    };

    let get_initial_state: Option<(&String, &Vec<Transition>)> = m.transitions.get_key_value(&m.initial);
    match get_initial_state {
        Some(initial_state) => {
            println!("{:#^80}", "");
            println!("{:#^80}", "");
            println!("{:^15}|{:^23}|{:^3}|{:^23}|{:^3}|{:^8}", "Ribbon", "Current State", "R", "New State", "W",  "action");
            let final_ribbon: Result<String, String> = machine_step(m, input.to_string(), 0, initial_state);
            match final_ribbon {
                Ok(r) => return Ok(r),
                Err(error) => { return Err(format!("MACHINE ERROR: {}", error));},
            };
        }, 
        None => { return Err(format!("Something went very wrong"));},
    };
}