use crate::machine_parser::Machine;

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

pub fn machine_start(m: &Machine, input: String)-> Result<String, String> {

    //check ribbon
    let input_test = input_checker(&m, &input);
    match input_test {
        Ok(()) => {},
        Err(error) => { return Err(format!("INPUT ERROR: [{}]", error));},
    };


    // println!("test: [{}]", &input.chars().nth(0).unwrap());

    // Err(format!("Some crap went wrong"))

    Ok(format!("<<Final Ribbon Placeholder>>"))
}