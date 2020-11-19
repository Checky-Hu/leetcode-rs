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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result_mut: &mut Box<ListNode> = &mut result;
        let mut is_odd: bool = true;
        let mut s: Option<&Box<ListNode>> = head.as_ref();
        while let Some(x) = s {
            if is_odd {
                result_mut.next = Some(Box::new(ListNode::new(x.val)));
                result_mut = result_mut.next.as_mut().unwrap();
                is_odd = false;
            } else {
                is_odd = true;
            }
            s = x.next.as_ref();
        }
        is_odd = true;
        s = head.as_ref();
        while let Some(x) = s {
            if is_odd {
                is_odd = false;
            } else {
                result_mut.next = Some(Box::new(ListNode::new(x.val)));
                result_mut = result_mut.next.as_mut().unwrap();
                is_odd = true;
            }
            s = x.next.as_ref();
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

    let result: Option<Box<ListNode>> = Solution::odd_even_list(head.next);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
