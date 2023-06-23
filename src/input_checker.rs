use crate::machine_parser::Machine;
use crate::utils::{check_contains_all};

pub fn check_input(m: &Machine, input: &String) -> Result<(), String> {
    if input.is_empty() {
        Err(format!("Input is empty"))
    } else if input.contains(m.blank) {
        Err(format!("blank char [{}] not allowed in input", m.blank))
    } else if let Some(not_in) = check_contains_all(&input.chars().collect::<Vec<char>>(), &m.alphabet) {
        Err(format!("symbol [{}] is not part of alphabet", not_in))
    } else {
        Ok(())
    }
}
