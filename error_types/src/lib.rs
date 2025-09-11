use chrono::NaiveDateTime;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
    pub form_values : (String,String),
    pub date : String ,
    pub err : String,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self { form_values: (field_name.to_string() , field_value), date: chrono::Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string()  , err: err.to_string() }
    }
}




impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let date  =  chrono::Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string() ;
        // todo!()
        if self.name.len() < 1 {
           return   Err(FormError{err:"Username is empty".to_string() , form_values: ("Username".to_string(),self.name.to_string()) , date :date} );
        }

        if self.password.len() < 8 {
             return  Err(FormError{err:"Password should be at least 8 characters long".to_string() , form_values: ("Password".to_string(),self.password.to_string()) , date : date} ) ;
        }

        let isdigit =  self.password.chars().any(|c| c.is_ascii_digit());
        let isletter =  self.password.chars().any(|c| c.is_ascii_alphabetic());
        let ispenc =  self.password.chars().any(|c| c.is_ascii_punctuation() || (c as u8 >= 0x24 && c as u8 <= 0x2F)  /*|| c.is_ascii_symbol() */);

        if !isdigit && !isletter && !ispenc {
             return  Err(FormError{err:"Password should be a combination of ASCII numbers, letters and symbols".to_string() , form_values: ("Password".to_string(),self.password.to_string()) , date :date} ) ;
        }

         Ok(())
    }

   
}