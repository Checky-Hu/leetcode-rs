use std::env;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast: Option<&Box<ListNode>> = head.as_ref();
        for _i in 0..n {
            if let Some(x) = fast {
                fast = x.next.as_ref();
            }
        }
        let mut slow: Option<&Box<ListNode>> = head.as_ref();
        let mut results: Box<ListNode> = Box::new(ListNode::new(0));
        let mut pointer: &mut Box<ListNode> = &mut results;
        while let (Some(x), Some(y)) = (fast.as_ref(), slow.as_ref()) {
            pointer.next = Some(Box::new(ListNode::new(y.val)));
            pointer = pointer.next.as_mut().unwrap();
            fast = x.next.as_ref();
            slow = y.next.as_ref();
        }
        if let Some(x) = slow {
            slow = x.next.as_ref();
            while let Some(y) = slow.as_ref() {
                pointer.next = Some(Box::new(ListNode::new(y.val)));
                pointer = pointer.next.as_mut().unwrap();
                slow = y.next.as_ref();
            }
        }
        results.next
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
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

    let result: Option<Box<ListNode>> = Solution::remove_nth_from_end(head.next, n);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
