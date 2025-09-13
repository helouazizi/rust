pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let mut res: usize = 0;
    let lenght = num_str.len();

    for c in num_str.chars() {
        
        let n = c.to_digit(10).unwrap() as usize;
        res += n.pow(lenght as u32);
    }
    if res == num as usize { true } else { false }
}
