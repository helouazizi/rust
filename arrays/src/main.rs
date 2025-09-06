use arrays::*;

fn main() {
    let a:[i32;10] = (1..=10).collect::<Vec<_>>().try_into().unwrap();
    let b: [i32; 10] = [5;10];

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}