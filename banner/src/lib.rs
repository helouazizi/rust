use std::{collections::HashMap, num::ParseFloatError};
// lqst co,,it 
pub struct Flag {
    // expected public fields
    short_hand: String,

    long_hand: String,

    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d:  &str) -> Self {
        Self{ short_hand : format!("-{}",name.chars().next().unwrap_or_default()) , long_hand :  format!("-{}",name) , desc : d.to_string()}
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand , func);
        self.flags.insert(flag.long_hand , func);
         self.flags.insert(flag.desc , func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {

        if let Some(callback) =  self.flags.get(input) {
            match  callback(argv[0],argv[1]) {
                Ok(res) => Ok(res),
                Err(e) => Err(format!("{}",e.to_string()))
            }
        }else {
            Err("invalid float literal".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a64 : f64 = a.parse()?;
    let b64 : f64 = b.parse()?;
    let dev : f64 =  a64 /b64 ;
    Ok(dev.to_string())
}


pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a64 : f64 = a.parse()?;
    let b64 : f64 = b.parse()?;
    let dev : f64 =  a64 % b64 ;
    Ok(dev.to_string())
}



 
