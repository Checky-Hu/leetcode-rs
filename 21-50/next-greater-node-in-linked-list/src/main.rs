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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut len: usize = 0;
        let mut vec: Vec<i32> = Vec::new();
        let mut t: Option<&Box<ListNode>> = head.as_ref();
        while let Some(x) = t {
            len += 1;
            vec.push(x.val);
            t = x.next.as_ref();
        }
        let mut result: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            let mut index: usize = len;
            for j in (i + 1)..len {
                if vec[j] > vec[i] {
                    index = j;
                    break;
                }
            }
            if index < len {
                result.push(vec[index]);
            } else {
                result.push(0);
            }
        }
        result
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

    let result: Vec<i32> = Solution::next_larger_nodes(head.next);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
