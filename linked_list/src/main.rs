// linked_list/src/main.rs

fn main() {
    println!("Hello, world!");

    let list = ListNode::new();
    println!("{:?}", list);
}

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new() -> ListNode {
        ListNode { val: 0, next: None }
    }

    fn push_front(&self, item: i32) {
        for val in self.next.unwrap() {}
    }
}
