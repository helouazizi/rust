pub fn score(str : &str) -> u64{
   let mut  nmb : u64 = 0 ;
   for c in str.chars() {
     let ch: char = c.to_ascii_uppercase();
    match ch {
        'A'|'E'|'I'| 'O'| 'U'| 'L'| 'N'| 'R'| 'S'|'T' => nmb+= 1 ,
       'D'| 'G'	=> nmb+= 2,
       'B'| 'C'| 'M'|'P'=>nmb+= 3,
      'F'|'H'| 'V'| 'W'|'Y' => nmb+= 4 ,
      'K' =>  nmb+= 5 ,
       'J'|'X' => nmb+= 8 ,
       'Q'| 'Z' => nmb+= 10 ,
       _ => nmb += 0,

    }
   }
   nmb 

}