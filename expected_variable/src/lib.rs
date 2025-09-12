

use std::fmt::format;

use convert_case::{Case, Casing};
// use edit_distance::edit_distance;

pub fn expected_variable(str1 :&str , str2 :&str ) -> Option<String> {

    if str1.to_ascii_lowercase() != str1.to_case(Case::Snake).to_ascii_lowercase()
    && str1.to_ascii_lowercase() != str1.to_case(Case::Camel).to_ascii_lowercase(){
        return None;
    }

    let lowstr1 = str1.to_lowercase();
    let lowstr2 = str2.to_ascii_lowercase();

    let steps = edit_distance(&lowstr1,&lowstr2);
 let persent = ((str2.len() - steps) as f64 / str2.len() as f64) * 100.0;

      
    if persent < 50.0 {
      return  None;
    }


     Some(format!("{}%",(persent).round() as i64))


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



fn is_snake_case(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
        && !s.starts_with('_')
        && !s.ends_with('_')
        && !s.contains("__")
}



fn is_camel_case(s: &str) -> bool {
    s.chars().next().map(|c| c.is_ascii_lowercase()).unwrap_or(false)
        && !s.contains('_')
        && s.chars().any(|c| c.is_uppercase())
}
