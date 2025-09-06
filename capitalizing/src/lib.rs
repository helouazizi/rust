pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    let mut first = true;

    for c in input.chars() {
        if !c.is_whitespace() && first {
            res.push_str(&c.to_uppercase().collect::<String>());
            first = false;
        } else {
            res.push(c);
            if c == '\t' {
                first = true;
            }
        }
    }

    res
}


pub fn title_case(input: &str) -> String {
    let mut c= input.split(" ").collect::<Vec<&str>>();
     let res :String =  c.iter_mut().map(|elm|  capitalize_first(elm)).collect::<Vec<_>>().join(" "); 
     res
}
pub fn change_case(input: &str) -> String {

    input
        .chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<Vec<_>>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<Vec<_>>()
            } else {
                vec![c] 
            }
        })
        .collect()
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
