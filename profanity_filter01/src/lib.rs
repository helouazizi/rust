// profanity_filter01/src/lib.rs
// Empty file

pub fn check_ms(message: &str) -> Result<&str, &str> {
  if message.contains("stupid") || message.is_empty() {

     Err("ERROR: illegal")
    
  }else{
       Ok("hello there")
  }
}