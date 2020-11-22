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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut vec1: Vec<i32> = Vec::new();
        let mut s: Option<&Box<ListNode>> = l1.as_ref();
        while let Some(x) = s {
            vec1.insert(0, x.val);
            s = x.next.as_ref();
        }
        let mut vec2: Vec<i32> = Vec::new();
        s = l2.as_ref();
        while let Some(x) = s {
            vec2.insert(0, x.val);
            s = x.next.as_ref();
        }
        let (src, mut dst) = if vec1.len() > vec2.len() {
            (vec2, vec1)
        } else {
            (vec1, vec2)
        };
        let mut carry: i32 = 0;
        for (i, v) in dst.iter_mut().enumerate() {
            let t: i32 = *v + carry + if i < src.len() { src[i] } else { 0 };
            carry = t / 10;
            *v = t % 10;
        }
        if carry > 0 {
            dst.push(carry);
        }
        let mut i: usize = dst.len() - 1;
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result_mut: &mut Box<ListNode> = &mut result;
        loop {
            result_mut.next = Some(Box::new(ListNode::new(dst[i])));
            result_mut = result_mut.next.as_mut().unwrap();
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result.next
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: i32 = 0;
    let mut count: i32 = 0;
    let mut l1: Box<ListNode> = Box::new(ListNode::new(0));
    let mut l1_mut: &mut Box<ListNode> = &mut l1;
    let mut l2: Box<ListNode> = Box::new(ListNode::new(0));
    let mut l2_mut: &mut Box<ListNode> = &mut l2;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                if count < len {
                    l1_mut.next = Some(Box::new(ListNode::new(val)));
                    l1_mut = l1_mut.next.as_mut().unwrap();
                    count += 1;
                } else {
                    l2_mut.next = Some(Box::new(ListNode::new(val)));
                    l2_mut = l2_mut.next.as_mut().unwrap();
                }
            }
        }
    }

    if len == 0 || ret == 0 {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result: Option<Box<ListNode>> = Solution::add_two_numbers(l1.next, l2.next);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
