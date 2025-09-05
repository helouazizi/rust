pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn delete_and_backspace(s: &mut String) {
 
    let chars : Vec<char> = s.chars().collect();
    let mut  res : String = String::new();
    let mut i = 0 ;
     while i < chars.len() {
        match chars[i] {
            '-' => {
                if !res.is_empty() {
                    res.pop();
                }
                i+= 1;
            }
            '+' => {

                let mut count =  0;
                while i < chars.len() && chars[i] == '+' {
                    count += 1;
                    i+= 1;
                }
                i+= count
             }
             c => {
                res.push(c);
                i+= 1
             }
        }
     }
     s.clear();
     s.push_str(&res);
}

pub fn do_operations(v: &mut [String]) {
    for op in v.iter_mut() {
        let operator_pos = op.find(|c| c == '+' || c == '-').expect("No operator found");
        let operator = op.chars().nth(operator_pos).unwrap();

        let num1: i32 = op[..operator_pos].parse().unwrap();
        let num2: i32 = op[operator_pos + 1..].parse().unwrap();

        let res = if operator == '+' { num1 + num2 } else { num1 - num2 };
        *op = res.to_string();
    }
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
