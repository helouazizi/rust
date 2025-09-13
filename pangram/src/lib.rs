pub fn is_pangram(s: &str) -> bool {
    let mut nmb = 0 ;
    let arr = ['a','b','c', 'd', 'e', 'f', 'j', 'h', 'i','g' ,'k' , 'l' , 'm' , 'n' ,'o' ,'p' ,'q' ,'r' ,'s' ,'t' ,'u','v','w' ,'x', 'y', 'z'];
     for c in arr {
        if s.to_ascii_lowercase().contains(c) {
            nmb+=1
        }

   
    
   }
   // arr.len() to make it dynamically
   if nmb < arr.len() {
    false
   } else {
    true
   }
   
}