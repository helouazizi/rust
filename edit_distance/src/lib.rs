pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn edit_distance(source: &str, target: &str) -> usize {


    // let detect if one of them is epty 
    if source.is_empty() {
        return target.len()
    }
    if target.is_empty(){
        return source.len()
    }

    let s =  source.len();
    let t =target.len();


    let s1 : Vec<char> = source.chars().collect();
    let t1 : Vec<char> = target.chars().collect();
    // println!("{:?}  {:?}" , s1 , t1);

    let table = vec![vec![0;t+1] ;s+1];
    println!("{:?}  " , table);

    100
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
