pub fn add(left: usize, right: usize) -> usize {
    left + right
}



pub fn nbr_function(c: i32) -> (i32, f64, f64) {
}

pub fn str_function(a: String) -> (String, String) {
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
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
