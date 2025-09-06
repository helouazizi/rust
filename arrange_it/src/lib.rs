pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn arrange_phrase(phrase: &str) -> String {
    let mut  buf : Vec<&str> = phrase.split(' ').collect();
    buf.sort_by_key(|c| find_digit(c));

    let res : String = buf.join(" ");
    res
}

fn find_digit(str : &str)  -> i32 {
    let digit : String =  str.chars().filter(|c| c.is_ascii_digit()).collect();
    digit.parse::<i32>().unwrap_or(0)
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
