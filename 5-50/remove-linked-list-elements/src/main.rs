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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result_mut: &mut Box<ListNode> = &mut result;
        let mut pointer: Option<&Box<ListNode>> = head.as_ref();
        while let Some(x) = pointer {
            if x.val != val {
                result_mut.next = Some(Box::new(ListNode::new(x.val)));
                result_mut = result_mut.next.as_mut().unwrap();
            }
            pointer = x.next.as_ref();
        }
        result.next
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut val: i32 = 0;
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => val = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                head_mut.next = Some(Box::new(ListNode::new(num)));
                head_mut = head_mut.next.as_mut().unwrap();
            }
        }
    }

    if ret == 0 {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Option<Box<ListNode>> = Solution::remove_elements(head.next, val);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
