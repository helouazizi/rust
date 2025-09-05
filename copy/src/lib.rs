pub fn add(left: usize, right: usize) -> usize {
    left + right
}



pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let f = c as f64;
    (c,f.exp(),f.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let nums: Vec<&str> = a.split_whitespace().collect();

    let exps: Vec<f64> = nums
        .iter()
        .map(|num| num.parse::<f64>().unwrap().exp())
        .collect();

    let exp_str = exps
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    (a, exp_str)
}


pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {

    let exps: Vec<f64> = b
        .iter()
        .map(|num| (*num as f64).abs().ln())
        .collect();
    (b, exps)
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
