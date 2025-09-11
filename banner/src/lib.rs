use std::{collections::HashMap, num::ParseFloatError};

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



 


#[test]
fn test_simple() {
    let mut HANDLER = FlagsHandler { flags: HashMap::new() };
    for a in ["-d", "--division"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "2.0"]), Ok("0.5".to_owned()));
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(HANDLER.exec_func(a, &["2.0", "2.0"]), Ok("0".to_owned()));
    }

    for a in ["-d", "--division"] {
        assert_eq!(
            HANDLER.exec_func(a, &["12.323", "212.32"]),
            Ok("0.058039751318764134".to_owned())
        );
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(
            HANDLER.exec_func(a, &["12.323", "212.32"]),
            Ok("12.323".to_owned())
        );
    }
}

#[test]
fn test_edge_cases() {
        let mut HANDLER = FlagsHandler { flags: HashMap::new() };
    for a in ["-d", "--division"] {
        assert_eq!(
            HANDLER.exec_func(a, &["a", "2.0"]),
            Err("invalid float literal".to_owned())
        );
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(
            HANDLER.exec_func(a, &["2.0", "f"]),
            Err("invalid float literal".to_owned())
        );
    }

    for a in ["-d", "--division"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("inf".to_owned()));
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("NaN".to_owned()));
    } }