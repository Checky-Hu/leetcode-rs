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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // (is first, is unique, value)
        let mut prefix: (bool, bool, i32) = (true, false, 0);
        let mut vec: Vec<i32> = Vec::new();
        let mut pointer: Option<&Box<ListNode>> = head.as_ref();
        while let Some(x) = pointer {
            if prefix.0 {
                prefix.0 = false;
                prefix.1 = true;
                prefix.2 = x.val;
            } else if prefix.2 == x.val {
                prefix.1 = false;
            } else {
                if prefix.1 {
                    vec.push(prefix.2);
                }
                prefix.1 = true;
                prefix.2 = x.val;
            }
            pointer = x.next.as_ref();
        }
        if prefix.1 {
            vec.push(prefix.2);
        }
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result_mut: &mut Box<ListNode> = &mut result;
        for v in vec.iter() {
            result_mut.next = Some(Box::new(ListNode::new(*v)));
            result_mut = result_mut.next.as_mut().unwrap();
        }
        result.next
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

    let result: Option<Box<ListNode>> = Solution::delete_duplicates(head.next);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
