pub fn ribbon_checker(m: &Machine, input: String)-> Option<String> {
    for c in input.chars() { 
        if c == m.blank {
            return Some(format!("blank char [{}] not allowed in input"));
        }
        else if m.alphabet.iter().position(|x| x == c) == None {
            return Some(format!("symbol [{}] is not part of alphabet", c));
        }
    }
    return None;
}