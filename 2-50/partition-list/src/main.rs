use std::cmp::Ordering;
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head_mut: Option<Box<ListNode>> = head;
        let mut t: Option<&Box<ListNode>> = head_mut.as_ref();
        let mut i: usize = 0;
        let mut vec: Vec<(i32, usize)> = Vec::new();
        while let Some(y) = t {
            vec.push((y.val, i));
            i += 1;
            t = y.next.as_ref();
        }
        vec.sort_by(|a, b| {
            if a.0 >= x {
                if b.0 >= x {
                    a.1.cmp(&b.1)
                } else {
                    Ordering::Greater
                }
            } else if b.0 >= x {
                Ordering::Less
            } else {
                a.1.cmp(&b.1)
            }
        });
        i = 0;
        let mut result: Option<&mut Box<ListNode>> = head_mut.as_mut();
        while let Some(mut y) = result {
            y.val = vec[i].0;
            i += 1;
            result = y.next.as_mut();
        }
        head_mut
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: i32 = 0;
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => x = i32::from_str(&arg).expect("Error parse."),
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

    let result: Option<Box<ListNode>> = Solution::partition(head.next, x);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(y) = t {
        print!("{} ", y.val);
        t = y.next.as_ref();
    }
    println!();
}
