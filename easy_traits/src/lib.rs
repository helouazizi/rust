#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
     fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        Self { value: self.value.clone() }
     }
      fn append_number(&mut self, nb_to_append: f64) -> Self {
         let string = nb_to_append.to_string();
        self.value.push_str(&string);
        Self { value: self.value.clone() }
      }


      fn remove_punctuation_marks(&mut self) -> Self{
          println!("{}",self.value);
        let clean :String  = self.value.chars().filter(|c| *c == '-' || !c.is_ascii_punctuation()).collect();
        println!("{}",clean);
        self.value = clean ;
        Self { value:self.value.clone() }
      }

   
}