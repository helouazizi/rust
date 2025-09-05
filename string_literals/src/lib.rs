pub fn add(left: usize, right: usize) -> usize {
    left + right
}



pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
   let res : ( &str , &str) = v.split_at(2);
   res 
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("not found ")
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
