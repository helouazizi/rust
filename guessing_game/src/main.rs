use std::io;


fn main() {

    println!("Enter a number");

    let mut gess = String::new();
    io::stdin()
        .read_line(&mut gess)
        .expect("Fail to read the input");
    println!("you gessed {gess}");
}
