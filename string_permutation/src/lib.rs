use std::collections::HashMap;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s2.len() != s1.len(){
        return false
    }

    let mut s1_hash : HashMap<char,usize> = HashMap::new();

    let mut s2_hash : HashMap<char,usize> = HashMap::new();

    for c in s1.chars() {
       
        if s1_hash.contains_key(&c) {
            s1_hash.insert(c, s1_hash.get(&c).unwrap() +1);
        }else {
             s1_hash.insert(c,1);
        }
    }

    for c in s2.chars() {
       
        if s2_hash.contains_key(&c) {
            s2_hash.insert(c, s2_hash.get(&c).unwrap() +1);
        }else {
             s2_hash.insert(c,1);
        }
    }

    // println!("{:?}",s2_hash);

    for (k,v) in s1_hash {
        // println!("{:?} {:?}",k ,v);
        if  !s2_hash.contains_key(&k) ||  &v != s2_hash.get(&k).unwrap() {
            return false
        }
    }

    true
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
