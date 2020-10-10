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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head_mut: Option<Box<ListNode>> = head;
        let mut t: Option<&Box<ListNode>> = head_mut.as_ref();
        let mut vec: Vec<i32> = Vec::new();
        while let Some(x) = t {
            vec.push(x.val);
            t = x.next.as_ref();
        }
        let len: usize = vec.len();
        if len == 0 {
            return head_mut;
        }
        for _i in 0..(k as usize % len) {
            if let Some(x) = vec.pop() {
                vec.insert(0, x);
            }
        }
        let mut i: usize = 0;
        let mut result: Option<&mut Box<ListNode>> = head_mut.as_mut();
        while let Some(mut x) = result {
            x.val = vec[i];
            i += 1;
            result = x.next.as_mut();
        }
        head_mut
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                head_mut.next = Some(Box::new(ListNode::new(val)));
                head_mut = head_mut.next.as_mut().unwrap();
            }
        }
    }

    if ret == 0 {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Option<Box<ListNode>> = Solution::rotate_right(head.next, k);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
