use std::env;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(v: i32) -> Self {
        ListNode { val: v, next: None }
    }
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vec: Vec<i32> = Vec::new();
        let mut t: Option<&Box<ListNode>> = head.as_ref();
        while let Some(x) = t {
            vec.push(x.val);
            t = x.next.as_ref();
        }
        if vec.is_empty() {
            return true;
        }

        let mut i: usize = 0;
        let mut j: usize = vec.len() - 1;
        while i < j {
            if vec[i] != vec[j] {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                head_mut.next = Some(Box::new(ListNode::new(val)));
                head_mut = head_mut.next.as_mut().unwrap();
            }
        }
    }

    if ret == 0 {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Is Palindrome linked list: {}",
        Solution::is_palindrome(head.next)
    );
}
