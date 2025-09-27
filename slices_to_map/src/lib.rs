use std::collections::HashMap;
 use std::hash::Hash;

pub fn slices_to_map<'a, T, U>(key: &'a [T], value: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: Eq + Hash,
{
    let mut length = key.len();
    if length > value.len() {
        length = value.len();
    }

    let mut res: HashMap<&'a T, &'a U> = HashMap::new();
    for i in 0..length {
        res.insert(&key[i], &value[i]);
    }

    res
}