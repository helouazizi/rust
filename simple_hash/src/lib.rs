use std::collections::HashMap;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn word_frequency_counter<'a>(words: &[&'a str])  -> HashMap<&'a str, usize> {
    let mut  hash: HashMap<&str,usize> = HashMap::new();
    for word in words {
        // println!("{}",word);
        if hash.contains_key(word) {
            hash.insert(word,hash.get(word).unwrap()+1);
        }else {
            hash.insert(word,1);
        }
        
    }
    hash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
frequency_count.len()
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
