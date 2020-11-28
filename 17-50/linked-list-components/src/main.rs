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
    pub fn num_components(head: Option<Box<ListNode>>, g: Vec<i32>) -> i32 {
        let mut len: usize = 0;
        let mut t: Option<&Box<ListNode>> = head.as_ref();
        while let Some(x) = t {
            len += 1;
            t = x.next.as_ref();
        }
        let mut vec: Vec<bool> = vec![false; len];
        for v in g.iter() {
            vec[*v as usize] = true;
        }
        let mut result: i32 = 0;
        let mut is_linked: bool = false;
        t = head.as_ref();
        while let Some(x) = t {
            if vec[x.val as usize] {
                if !is_linked {
                    result += 1;
                    is_linked = true;
                }
            } else {
                is_linked = false;
            }
            t = x.next.as_ref();
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: usize = 0;
    let mut g: Vec<i32> = Vec::new();
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                if g.len() == n {
                    head_mut.next = Some(Box::new(ListNode::new(val)));
                    head_mut = head_mut.next.as_mut().unwrap();
                } else {
                    g.push(val);
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    println!("Components: {}", Solution::num_components(head.next, g));
}
