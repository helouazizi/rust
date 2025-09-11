use std::{error::Error, fmt::Display};
use std::fmt;

#[derive(Debug)]
pub enum ParseErr {
    // expected public fields
    Empty,

    Malformed(Box<dyn Error>),
    
}

impl Display for ParseErr {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f,"Input is empty"),
            ParseErr::Malformed(e) => write!(f,"Malformed input: {}", e)
        }
    }
}

impl Error for ParseErr {
   fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Malformed(e) => Some(&**e),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
     pub  child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Read error: {}", self.child_err)
    }
    
}

impl Error for ReadErr {
   fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.child_err)
    }
}