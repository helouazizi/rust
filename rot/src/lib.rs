pub fn rotate(input: &str, key: i8) -> String {
let arr: Vec<char> = ('a'..='z').collect();

    let mut is_upper = false;
    let mut res = String::new();
    for  c in input.chars() {
        let mut  lowerc = c;
        if c.is_ascii_alphabetic() {
            if c.is_ascii_uppercase() {
               lowerc = c.to_ascii_lowercase();
                is_upper = true;
            }
            
            let  degit = arr.iter().position(|x|{*x==lowerc}).unwrap();
            let mut  k  = key ;
            if key <0 {
                k += 26 ;
                
            }
            let   cha = arr[(degit+k as usize)%26];
            if is_upper{
                res.push(cha.to_ascii_uppercase());
                is_upper = false;

            } else {
                res.push(cha); 

            }

        } else {
            res.push(c);

        }
    }

    res
}