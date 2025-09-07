use std::collections::HashMap;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn mean(list: &[i32]) -> f64 {
  
    let sum  :i32 =  list.iter().sum();

    (sum as f64)/ (list.len() as f64)
  
}

pub fn median(list: &[i32]) -> i32 {

    let mut test = list.to_vec();
    test.sort();

    if test.len() % 2 == 0 {
        return   (test[(test.len()/2)-1] +  test[test.len()/2]) /2
    }

     test[test.len()/2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut  hash :  HashMap<&i32,usize> = HashMap::new();
    for num  in list {
        if hash.contains_key(&num) {
            hash.insert(num , hash.get(&num).unwrap()+1);
        }else{
            hash.insert(num,1);
        }
    }
    println!("{:?}", hash);
    let max = hash.iter().max_by_key(|(_,&val)| val);
    if let Some((k,v)) =  max {
        return **k 
    }
    0
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
