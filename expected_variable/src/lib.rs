

use std::fmt::format;

use convert_case::{Case, Casing};
use edit_distance::edit_distance;

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
