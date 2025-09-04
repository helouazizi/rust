pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn celsius_to_fahrenheit(c: f32) -> f32 {
   let res =  (c * 9.0 / 5.0) + 32.0;   
     // round to 2 decimals
    (res * 100.0).round() / 100.0               
}

pub fn fahrenheit_to_celsius(f: f32) -> f32 {
    ((f - 32.0) * 5.0 / 9.0 * 100.0).round() / 100.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(27.0);
        assert_eq!(result, 4.0);
    }
}
