pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn factorial(num: u64) -> u64 {

    if num == 0 || num == 1 {
        return  1
    }
    let mut res = num;
    for  i in 1..num {
        res = res * i
    }
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
