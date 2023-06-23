use crate::machine_parser::Machine;

pub fn check_input(m: &Machine, input: &String) -> Result<(), String> {
    if input.is_empty() {
        return Err(format!("Input is empty"));
    }

    for c in input.chars() {
        if c == m.blank {
            return Err(format!("blank char [{}] not allowed in input", m.blank));
        } else if m.alphabet.iter().position(|x| x == &c) == None {
            return Err(format!("symbol [{}] is not part of alphabet", c));
        }
    }
    Ok(())
}
