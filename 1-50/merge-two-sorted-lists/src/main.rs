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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1_mut, mut l2_mut) = (l1, l2);
        let mut header: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result: &mut Box<ListNode> = &mut header;
        while let (Some(x), Some(y)) = (l1_mut.as_ref(), l2_mut.as_ref()) {
            if x.val < y.val {
                result.next = l1_mut;
                result = result.next.as_mut().unwrap();
                l1_mut = result.next.take();
            } else {
                result.next = l2_mut;
                result = result.next.as_mut().unwrap();
                l2_mut = result.next.take();
            }
        }
        result.next = if l1_mut.is_some() { l1_mut } else { l2_mut };
        header.next
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut count: usize = 0;
    let mut l1: Box<ListNode> = Box::new(ListNode::new(0));
    let mut l1_mut: &mut Box<ListNode> = &mut l1;
    let mut l2: Box<ListNode> = Box::new(ListNode::new(0));
    let mut l2_mut: &mut Box<ListNode> = &mut l2;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                if count == len {
                    l2_mut.next = Some(Box::new(ListNode::new(val)));
                    l2_mut = l2_mut.next.as_mut().unwrap();
                } else {
                    l1_mut.next = Some(Box::new(ListNode::new(val)));
                    l1_mut = l1_mut.next.as_mut().unwrap();
                    count += 1;
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result: Option<Box<ListNode>> = Solution::merge_two_lists(l1.next, l2.next);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
