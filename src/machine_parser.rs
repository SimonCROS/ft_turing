use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Transition {
    pub read: char,
    pub to_state: String,
    pub write: char,
    pub action: String,
}

#[derive(Deserialize)]
pub struct Machine {
    pub name: String,
    pub alphabet: Vec<char>,
    pub blank: char,
    pub states: Vec<String>,
    pub initial: String,
    pub finals: Vec<String>,
    pub transitions: HashMap<String, Vec<Transition>>,
}

fn check_duplicates<T>(list: &Vec<T>) -> Option<T>
where
    T: PartialEq + Clone,
{
    match (1..list.len()).find(|i| list[*i..].contains(&list[i - 1])) {
        Some(duplicated) => Some(list[duplicated].clone()),
        None => None,
    }
}

fn check_duplicates_on<T, F: Fn(&T, &T) -> bool>(list: &Vec<T>, f: F) -> Option<usize>
{
    match (1..list.len()).find(|i| list[*i..].iter().find(|el| f(&list[i - 1], el)).is_some()) {
        Some(duplicated) => Some(duplicated),
        None => None,
    }
}

fn check_contains_all<T>(needle: &Vec<T>, haystack: &Vec<T>) -> Option<T>
where
    T: PartialEq + Clone,
{
    match needle.iter().find(|el| !haystack.contains(el)) {
        Some(not_in) => Some(not_in.clone()),
        None => None,
    }
}

pub fn machine_checker(m: &Machine) -> Result<(), String> {
    if m.alphabet.len() < 2 {
        Err(format!("a minimum of 2 alphabet symbol are required (blank + 1 other minimum)"))
    } else if m.states.len() < 2 {
        Err(format!("a minimum of 2 states are required (initial + 1 final minimum)"))
    } else if m.finals.len() < 1 {
        Err(format!("at least one final state is required"))
    } else if !m.alphabet.contains(&m.blank) {
        Err(format!("blank symbol [{}] is not part of alphabet", m.blank))
    } else if !m.states.contains(&m.initial) {
        Err(format!("initial state [{}] is not part of states", m.initial))
    } else if m.finals.contains(&m.initial) {
        Err(format!("initial state [{}] is also a final state", m.initial))
    } else if let Some(not_in) = check_contains_all(&m.finals, &m.states) {
        Err(format!("final state [{}] is not part of states", not_in))
    } else if let Some(duplicated) = check_duplicates(&m.alphabet) {
        Err(format!("duplicate alphabet symbol [{}]", duplicated))
    } else if let Some(duplicated) = check_duplicates(&m.states) {
        Err(format!("duplicate state [{}]", duplicated))
    } else {
        check_transitions(m)
    }
}

fn check_transition(m: &Machine, name: &String, transition: &Transition) -> Result<(), String> {
    if !matches!(transition.action.as_str(), "LEFT" | "RIGHT") {
        Err(format!("statement action [{}] from [{}] read [{}] should be [RIGHT] or [LEFT] only", transition.action, name, transition.read))
    } else if !m.states.contains(&transition.to_state) {
        Err(format!("statement to_state [{}] from [{}] read [{}] is not part of states", transition.to_state, name, transition.read))
    } else if !m.alphabet.contains(&transition.read) {
        Err(format!("statement read [{}] from [{}] read [{}] is not part of alphabet", transition.read, name, transition.read))
    // } else if !m.alphabet.contains(&transition.write) {
    //     Err(format!("statement write [{}] from [{}] read [{}] is not part of alphabet", transition.write, name, transition.read))
    } else {
        Ok(())
    }
}

fn check_state_transitions(m: &Machine, name: &String, transitions: &Vec<Transition>) -> Result<(), String> {
    if let Some(duplicated_index) = check_duplicates_on(transitions, |l, r| l.read == r.read) {
        Err(format!("multiple statement for read [{}] in transition [{}]", transitions[duplicated_index].read, name))
    } else {
        transitions.iter().try_for_each(|item| check_transition(m, name, item))
    }
}

fn check_transitions(m: &Machine) -> Result<(), String> {
    let transitions_keys: Vec<String> = m.transitions.keys().cloned().collect();
    let not_final_states: Vec<String> = m.states.iter().filter(|el| !m.finals.contains(el)).cloned().collect();

    if let Some(duplicated) = check_duplicates(&transitions_keys) {
        Err(format!("duplicate transition [{}]", duplicated))
    } else if let Some(found) = transitions_keys.iter().find(|el| m.finals.contains(*el)) {
        Err(format!("transition [{}] is a final state and cannot have transition", found))
    } else if let Some(not_in) = check_contains_all(&transitions_keys, &not_final_states) {
        Err(format!("transition [{}] is not part of states", not_in))
    } else if let Some(not_in) = check_contains_all(&not_final_states, &transitions_keys) {
        Err(format!("state [{}] is not implemented in transitions", not_in))
    } else if m.transitions.iter().all(|(_, trs)| trs.iter().all(|tr| !m.finals.contains(&tr.to_state))) {
        Err(format!("no final state is used in any transition"))
    } else {
        m.transitions.iter().try_for_each(|(k, v)| check_state_transitions(m, k, v))
    }
}
