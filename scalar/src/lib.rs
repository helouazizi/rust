pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sum(a: i32, b: i32) -> i32 {
    let res = a + b;
    res
}


pub fn diff(a: i32, b: i32) -> i32 {
5
}

pub fn pro(a: i32, b: i32) -> i32 {
5
}

pub fn quo(a: i32, b: i32) -> i32{
5
}

pub fn rem(a: i32, b: i32) -> i32{
5
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
