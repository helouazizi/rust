pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sum(a: i32, b: i32) -> i32 {
    let res = a + b;
    res
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
