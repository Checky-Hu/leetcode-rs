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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_mut: Option<Box<ListNode>> = head;
        let mut result: Option<&mut Box<ListNode>> = head_mut.as_mut();
        while let Some(mut x) = result {
            if let Some(mut y) = x.next.as_mut() {
                let t: i32 = x.val;
                x.val = y.val;
                y.val = t;
                result = y.next.as_mut();
            } else {
                break;
            }
        }
        head_mut
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

    let result: Option<Box<ListNode>> = Solution::swap_pairs(head.next);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
