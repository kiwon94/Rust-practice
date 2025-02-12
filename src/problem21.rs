// Problem 21 - Convert Binary Number in a Linked List to Integer

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut binary_digits = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        binary_digits.push(node.val.to_string());
        current = node.next;
    }

    let binary_str = binary_digits.join("");
    i32::from_str_radix(&binary_str, 2).expect("Failed to convert binary string to integer")
}

pub fn solve() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));
    let output = get_decimal_value(head);
    println!("Integer value: {output}");
}
