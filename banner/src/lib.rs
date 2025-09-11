 use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String, 
    long_hand: String,
    desc: String
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
       Self { short_hand: format!("-{}",name.chars().nth(0).unwrap()), long_hand: format!("--{name}"), desc: format!("{d}") }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
        
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
         if let  Some(callback) = self.flags.get(input){
            match callback(argv[0], argv[1]) {
                Ok(res) => Ok(res),
                Err(e) => Err(format!("{}", e.to_string())),
            } 
        } else {
                Err("invalid float literal".to_string())

            }

         }
       
           
        
         
    }


pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_parse: f64 = a.parse()?;
    let b_parse: f64 = b.parse()?;
    let c = a_parse/b_parse ;
    Ok(c.to_string())

}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_parse: f64 = a.parse()?;
    let b_parse: f64 = b.parse()?;
    let c = a_parse%b_parse ;
    Ok(c.to_string())
}