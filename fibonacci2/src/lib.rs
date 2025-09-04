pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n
    }
    fibonacci(n-1) + fibonacci(n-2)

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
