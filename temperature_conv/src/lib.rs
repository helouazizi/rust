pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let c =(f-32.0) * (5.0 /9.0);
    (c*100.0).round() / 100.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let f =   (c+32.0)*(9.0/5.0) ;
    (f*100.0).round() / 100.0
  
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
