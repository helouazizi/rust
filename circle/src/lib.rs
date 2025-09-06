pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center //..
	pub radius //..
}

impl Circle {
    // ...
}

#[derive(Debug, Clone, Copy)]
pub struct Point(/* */);

impl Point {
    // ...
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
