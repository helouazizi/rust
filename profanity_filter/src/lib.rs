// Empty file

pub fn check_ms(message: &str) -> Result<&str, &str> {
  if message.contains("stupid") || message.is_empty() {
    Ok("hello there")
    
  }else{
    Err("ERROR: illegal")
  }
}