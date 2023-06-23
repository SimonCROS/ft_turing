use crate::machine_parser::Machine;
use crate::machine_parser::Transition;
use crate::print::print_output_header;
use colored::Colorize;
use tailcall::tailcall;

const TRANSITION_NOT_FOUND: &str = "Transition not found";
const UNKNOWN_ACTION: &str = "Unknown action";

fn machine_check_transitions(state: &Vec<Transition>, read: char) -> Result<&Transition, ()> {
    for t in state.iter() {
        if t.read == read {
            return Ok(t);
        }
    }
    return Err(());
}

#[tailcall]
fn machine_step(m: &Machine, ribbon: String, index: usize, state: (&String, &Vec<Transition>)) -> Result<String, String> {
    let c: char = ribbon.chars().nth(index).unwrap();

    let get_transition: Result<&Transition, ()> = machine_check_transitions(state.1, c);
    match get_transition {
        Ok(t) => {
            let left = &ribbon[..index];
            let right = &ribbon[index + 1..];

            let colored_ribbon: String = format!("{}{}{}", left, c.to_string().red(), right);
            println!("{:^15} |{:^23}R{:^3}>{:^23}W{:^3}|{:^8}", colored_ribbon, state.0, t.read, t.to_state, t.write, t.action);

            if m.finals.contains(&t.to_state) {
                return Ok(format!("{}{}{}", left, t.write, right));
            }

            let new_state = m.transitions.get_key_value(&t.to_state).expect(TRANSITION_NOT_FOUND);
            match (t.action.as_str(), index) {
                ("LEFT", 0) => machine_step(m, format!("{}{}{}{}", m.blank, left, t.write, right), 0, new_state),
                ("LEFT", _) => machine_step(m, format!("{}{}{}", left, t.write, right), index - 1, new_state),
                ("RIGHT", i) if i + 1 == ribbon.len() => machine_step(m, format!("{}{}{}{}", left, t.write, right, &m.blank), index + 1, new_state),
                ("RIGHT", _) => machine_step(m, format!("{}{}{}", left, t.write, right), index + 1, new_state),
                _ => panic!("{}", UNKNOWN_ACTION)
            }
        }
        Err(()) => Err(format!("State [{}] has not transition for read [{}], Aborting", state.0, ribbon.chars().nth(index).unwrap()))
    }
}

pub fn machine_start(m: &Machine, input: &String) -> Result<String, String> {
    let initial_state: (&String, &Vec<Transition>) = m.transitions.get_key_value(&m.initial).expect(TRANSITION_NOT_FOUND);
    print_output_header();
    machine_step(m, input.to_string(), 0, initial_state)
}
