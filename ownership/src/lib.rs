pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn first_subword(s: String) -> String {

    for (pos , c) in s.chars().enumerate().skip(1){
        if c == '_' || c.is_uppercase(){
            return s[..pos].to_string();
        }
    }

    s.to_string()
    
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
