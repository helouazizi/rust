pub fn search(array: &[i32], key: i32) -> Option<usize> {
    Some(array.iter().rposition(|&x| x == key)? )
}