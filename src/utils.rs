pub fn check_duplicates<T>(list: &Vec<T>) -> Option<T>
where
    T: PartialEq + Clone,
{
    match (1..list.len()).find(|i| list[*i..].contains(&list[i - 1])) {
        Some(duplicated) => Some(list[duplicated].clone()),
        None => None,
    }
}

pub fn check_duplicates_on<T, F: Fn(&T, &T) -> bool>(list: &Vec<T>, f: F) -> Option<usize>
{
    match (1..list.len()).find(|i| list[*i..].iter().find(|el| f(&list[i - 1], el)).is_some()) {
        Some(duplicated) => Some(duplicated),
        None => None,
    }
}

pub fn check_contains_all<T>(needle: &Vec<T>, haystack: &Vec<T>) -> Option<T>
where
    T: PartialEq + Clone,
{
    match needle.iter().find(|el| !haystack.contains(el)) {
        Some(not_in) => Some(not_in.clone()),
        None => None,
    }
}
