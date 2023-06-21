use crate::machine_parser::Machine;
use crate::machine_parser::Transition;
use colored::Colorize;

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



fn machine_step(m: &Machine, ribbon: &String, index: usize, state: (&String, &Vec<Transition>))-> Result<String, String> {
    //recursive
    //check transitions -> make new string and new index
    //if final state (HALT) Ok(final ribbon)
    //if transition doesnt exist Err("no transi found")
    //other types of errors ?
    let c: char = ribbon.chars().nth(index).unwrap(); 

    //do the printing somewhere
    let transi = machine_check_transitions(state.1, c);
    match transi {
        Ok(t) => {
            //do more stuff
            //print info
            // println!("[{}]: state [{}] read [{}] -> new state [{}] write [{}] action [{}]", ribbon, state.0, t.read, t.to_state, t.write, t.action);
            //prepare new string
            //             let slice = &"Golden Eagle"[..6];








            let left = &ribbon[..index];
            let right = &ribbon[index+1..];
            // print!("{}{}{}", left, c.to_string().red() , right );
//TODO find way to put color while keeping column size
let test = format!("{}{}{}", left, c.to_string().red() , right );
println!("{:^15}|{:^23}R{:^3}>{:^23}W{:^3}|{:^8}", test, state.0, t.read, t.to_state, t.write, t.action);
// let test_a = format!("{:^15}", test);


// println!("{:^15}|{:^23}R{:^3}>{:^23}W{:^3}|{:^8}", ribbon, state.0, t.read, t.to_state, t.write, t.action);









            if m.finals.iter().position(|x| x == &t.to_state) != None {
                //new state is final, make new string and quit Ok
                // let left = &ribbon[..index];
                // let right = &ribbon[index+1..];
                let new_ribbon = format!("{}{}{}", left, t.write, right);

                return Ok(format!("{}", new_ribbon));
            }
 
            //make new call 
            let get_new_state = m.transitions.get_key_value(&t.to_state);
            match get_new_state {
                Some(new_state) => {

                    // let left = &ribbon[..index];
                    // let right = &ribbon[index+1..];
                    // let new_string = format!("{}{}{}", left, t.write, right);
                    
                    if t.action == "LEFT" {
//TODO change this, would need unsigned but .nth wants a usize ? need testing
                        // if index as isize - 1 < 0 {
                        if index == 0 {
                            let new_index: usize = 0;
                            let new_ribbon = format!("{}{}{}{}",m.blank, left, t.write, right);

                            //obligation de gerer le return, recursion mal construite a cause du result ?
                           return machine_step(m, &new_ribbon, new_index, new_state);
                        }
                        else {
                            let new_index = index - 1;
                            let new_ribbon = format!("{}{}{}", left, t.write, right);
                            return machine_step(m, &new_ribbon, new_index, new_state);
                        }
                    }
                    else if t.action == "RIGHT" {  
                        //if index too big add a blank after the string
                        let new_index = index + 1;
                        if new_index == ribbon.len() {
                            let new_ribbon = format!("{}{}{}{}", left, t.write, right, &m.blank);
                            return machine_step(m, &new_ribbon, new_index, new_state);
                        }   
                        else {
                            let new_ribbon = format!("{}{}{}", left, t.write, right);
                            return machine_step(m, &new_ribbon, new_index, new_state);
                        }    

                    }
                    else { return Err(format!("Something went very wrong"));} 



                    
                }, 
                None => { return Err(format!("Something went very wrong"));},
            };
        },
        // Err(()) => {},
        //get state name ?
        Err(()) => { return Err(format!("State [{}] has not transition for read [{}], Aborting", state.0, ribbon.chars().nth(index).unwrap()));},
    };




    
    // println!("test: [{}]", &input.chars().nth(0).unwrap());
    // Ok(format!("<<Final Ribbon Placeholder>>"))
}

pub fn machine_start(m: &Machine, input: &String)-> Result<String, String> {

    //check ribbon
    let input_test = input_checker(&m, &input);
    match input_test {
        Ok(()) => {},
        Err(error) => { return Err(format!("INPUT ERROR: {}", error));},
    };

    let get_initial_state = m.transitions.get_key_value(&m.initial);
    //just in case as previous check should make sure it resolves Ok
    match get_initial_state {
        Some(initial_state) => {
            println!("{:#^80}", "");
            println!("{:#^80}", "");
            println!("{:^15}|{:^23}|{:^3}|{:^23}|{:^3}|{:^8}", "Ribbon", "Current State", "R", "New State", "W",  "action");
            let final_ribbon = machine_step(m, &input, 0, initial_state);
            match final_ribbon {
                Ok(r) => return Ok(r),
                Err(error) => { return Err(format!("MACHINE ERROR: [{}]", error));},
            };

        }, 
        None => { return Err(format!("Something went very wrong"));},
    };
    //first call (m, input, 0)


    // println!("test: [{}]", &input.chars().nth(0).unwrap());

    // Err(format!("Some crap went wrong"))

    // Ok(format!("<<Final Ribbon Placeholder>>"))
}