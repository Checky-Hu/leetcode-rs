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
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut sum: i32 = 0;
        let mut t: Option<&Box<ListNode>> = root.as_ref();
        while let Some(x) = t {
            sum += 1;
            t = x.next.as_ref();
        }
        let m: i32 = sum / k;
        let n: i32 = sum % k;
        let mut counts: i32 = 0;
        let mut result: Vec<Option<Box<ListNode>>> = Vec::new();
        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let mut head_mut: &mut Box<ListNode> = &mut head;
        t = root.as_ref();
        while let Some(x) = t {
            head_mut.next = Some(Box::new(ListNode::new(x.val)));
            counts += 1;
            let target: i32 = if result.len() < n as usize { m + 1 } else { m };
            if counts == target {
                result.push(head.next);
                head = Box::new(ListNode::new(0));
                head_mut = &mut head;
                counts = 0;
            } else {
                head_mut = head_mut.next.as_mut().unwrap();
            }
            t = x.next.as_ref();
        }
        while result.len() < k as usize {
            result.push(None);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut root: Box<ListNode> = Box::new(ListNode::new(0));
    let mut root_mut: &mut Box<ListNode> = &mut root;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                root_mut.next = Some(Box::new(ListNode::new(val)));
                root_mut = root_mut.next.as_mut().unwrap();
            }
        }
    }

    if ret == 0 {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Vec<Option<Box<ListNode>>> = Solution::split_list_to_parts(root.next, k);
    for r in result.iter() {
        let mut t: Option<&Box<ListNode>> = r.as_ref();
        while let Some(x) = t {
            print!("{} ", x.val);
            t = x.next.as_ref();
        }
        println!();
    }
}
