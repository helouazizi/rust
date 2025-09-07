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

    let mut  table = vec![vec![0;t+1] ;s+1];
    for i in 0..=s {
        table[i][0] = i;
    }
     for j in 0..=t {
        table[0][j] = j;
    }

    // lets fill the table herae 
    for i in 1..=s {
        for j in 1..=t {
            // conpare chrcter 
            if s1[i-1] == t1[j-1]{
                table[i][j] = table[i-1][j-1];
            }else {
                table[i][j] = 1 + std::cmp::min(
                    table[i-1][j-1],
                    std::cmp::min(table[i-1][j],table[i][j-1])
                )
            }
        }
    }


   

    table[s][t]
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
