use std::collections::HashMap;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let max =  h.iter().max_by_key(|(_,&val)| val);
    if  let Some((_,val)) = max {
            return  *val;
    }else{
        0
    }
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
