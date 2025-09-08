// add_two_number/src/main.rs

#[derive(Debug)]
pub struct ListNode {
    val : i32 ,
    next : Option<Box<ListNode>>,
}

impl ListNode {
    fn new(num :i32) -> ListNode {
        ListNode { val: num, next: None }
    }
}

fn to_list(vector:&Vec<i32>) -> Option<Box<ListNode>> {
    let mut list = None ;
    for &num in vector.iter().rev() {
        let mut node = ListNode::new(num) ;
        node.next = list ;
        list =Some(Box::new(node));

    }

    list
}

fn main() {
    println!("Hello, world!");
    // let test =  ListNode{val:0 , next : Some(Box::new(ListNode::new(1)))} ;
    let vector = vec![0,1,2,3,4,5,6,7,8,9];
    let list1 = to_list(&vector) ;
    let list2 = to_list(&vector);
    println!("{:?}",list1);

}

fn add_two_number(l1: ListNode, l2 : ListNode) -> Option<Box<ListNode>> {
    
    None
}
