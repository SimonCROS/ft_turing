pub fn ribbon_checker(m: &Machine, input: String)-> Result<(), String> {
    for c in input.chars() { 
        if c == m.blank {
            return Err(format!("blank char [{}] not allowed in input"));
        }
        else if m.alphabet.iter().position(|x| x == c) == None {
            return Err(format!("symbol [{}] is not part of alphabet", c));
        }
    }
    Ok(())
}
