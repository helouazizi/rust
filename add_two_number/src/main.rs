// add_two_number/src/main.rs

#[derive(Debug)]
pub struct ListNode {
    val : i32 ,
    next : Option<Box<ListNode>>,
}



fn main() {
    println!("Hello, world!");
    let test =  ListNode{val:0 , next : Some(Box::new(ListNode{val:1, next:None}))} ;
    println!("{:?}",test.next.unwrap().val);

}
