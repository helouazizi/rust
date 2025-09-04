pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn celsius_to_fahrenheit(c: f64) -> f64 {
 (c * 9.0 / 5.0) + 32.0 
               
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let c = (f - 32.0) * 5.0 / 9.0;
    let factor = 1_000_000_000_000_000.0; // 15 decimals
    (c * factor).round() / factor
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(20.0);
        assert_eq!(result, -6.666666666666666);
    }
}
