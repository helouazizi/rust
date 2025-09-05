pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn initials(names: Vec<&str>) -> Vec<String> {

    let  mut strings : Vec<String> = Vec::with_capacity(names.len());
    for name in names {
        strings.push(name.split_whitespace()
                         .filter_map(| word | word.chars().next())
                         .map(|c| format!("{}.",c))
                         .collect::<Vec<String>>()
                         .join(" ")                       
    )
    }
    strings

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
