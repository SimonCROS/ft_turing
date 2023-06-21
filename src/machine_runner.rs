use crate::machine_parser::Machine;
use crate::machine_parser::Transition;

fn input_checker(m: &Machine, input: &String)-> Result<(), String> {
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


fn machine_step(m: &Machine, ribbon: String, index: usize, state: &Vec<Transition>)-> Result<String, String> {
    //recursive
    //check transitions -> make new string and new index
    //if final state (HALT) Ok(final ribbon)
    //if transition doesnt exist Err("no transi found")
    //other types of errors ?

    //do the printing somewhere
    let transi = machine_check_transitions(state, ribbon.chars().nth(index).unwrap());
    match transi {
        Ok(t) => {
            //do more stuff
            //print info
            //prepare new string
            //make new call 
        },
        Err(()) => {},
        //get state name ?
        // Err(error) => { return Err(format!("State [{}] has not transition for read [{}], Aborting", XXX, ribbon.chars().nth(index).unwrap()));},
    };




    
    // println!("test: [{}]", &input.chars().nth(0).unwrap());
    Ok(format!("<<Final Ribbon Placeholder>>"))
}

pub fn machine_start(m: &Machine, input: String)-> Result<String, String> {

    //check ribbon
    let input_test = input_checker(&m, &input);
    match input_test {
        Ok(()) => {},
        Err(error) => { return Err(format!("INPUT ERROR: [{}]", error));},
    };

    let get_initial_state = m.transitions.get(&m.initial);
    //just in case as previous check should make sure it resolves Ok
    match get_initial_state {
        Some(initial_state) => {
            let final_ribbon = machine_step(m, input, 0, initial_state);
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

    Ok(format!("<<Final Ribbon Placeholder>>"))
}