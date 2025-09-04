
use std::io;


fn main() {

    let  mut input  =  String::new();
    let mut  tries  = 1 ; 
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error ");

        let answer = input.trim();
        if answer == "The letter e" {
            break;
        }
        tries += 1 ;
    }

    println!("Number of trials: {}",tries);


}
