// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

pub fn diff(a: i16, b: i16) -> String {
    match a.checked_sub(b) {
        Some(val) => val.to_string(),
        _none => "ERROR: attempt to subtract with overflow".to_string(),
    }
}



pub fn sum(a: u8, b:u8) -> u8 {
    let res =  a- b ;
    res
}

pub fn quo(a: f32, b: f32) -> f32 {
let res = a/b ;
res 
}

pub fn pro(a: i8, b: i8) -> i8{
let res =  a*b ;
res
}

pub fn rem(a: f32, b: f32) -> f32{
let res =  a%b ;
res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(5, 6);
        assert_eq!(result, 5);
    }
}
