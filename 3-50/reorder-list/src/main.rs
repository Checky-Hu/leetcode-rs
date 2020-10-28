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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut t: Option<&Box<ListNode>> = head.as_ref();
        let mut vec: Vec<i32> = Vec::new();
        while let Some(x) = t {
            vec.push(x.val);
            t = x.next.as_ref();
        }
        let mut left: usize = 0;
        let mut right: usize = vec.len() - 1;
        let mut use_left: bool = true;
        let mut result: Option<&mut Box<ListNode>> = head.as_mut();
        while let Some(mut x) = result {
            if use_left {
                x.val = vec[left];
                left += 1;
                use_left = false;
            } else {
                x.val = vec[right];
                right -= 1;
                use_left = true;
            }
            result = x.next.as_mut();
        }
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

    Solution::reorder_list(&mut head.next);
    let mut t: Option<&Box<ListNode>> = head.next.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
