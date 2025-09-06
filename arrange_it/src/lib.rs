pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn arrange_phrase(phrase: &str) -> String {
      let mut buf: Vec<&str> = phrase.split_whitespace().collect();
    buf.sort_by_key(|c| find_digit(c));

    let cleaned: Vec<String> = buf.iter()
        .map(|w| w.chars().filter(|c| !c.is_ascii_digit()).collect())
        .collect();

    cleaned.join(" ")
}

fn find_digit(s : &str)  -> i32 {
    let mut num = 0;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            num = num * 10 + d as i32;
        }
    }
    num
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
