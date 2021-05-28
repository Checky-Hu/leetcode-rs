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
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut vector: Vec<i32> = Vec::new();
        let mut point1: Option<&Box<ListNode>> = list1.as_ref();
        let mut index: i32 = 0;
        while let Some(x) = point1 {
            if index < a || index > b {
                vector.push(x.val);
            } else if index == a {
                let mut point2: Option<&Box<ListNode>> = list2.as_ref();
                while let Some(y) = point2 {
                    vector.push(y.val);
                    point2 = y.next.as_ref();
                }
            }
            point1 = x.next.as_ref();
            index += 1;
        }
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result_mut: &mut Box<ListNode> = &mut result;
        for v in vector {
            result_mut.next = Some(Box::new(ListNode::new(v)));
            result_mut = result_mut.next.as_mut().unwrap();
        }
        result.next
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut len: i32 = 0;
    let mut count: i32 = 0;
    let mut list1: Box<ListNode> = Box::new(ListNode::new(0));
    let mut list1_mut: &mut Box<ListNode> = &mut list1;
    let mut list2: Box<ListNode> = Box::new(ListNode::new(0));
    let mut list2_mut: &mut Box<ListNode> = &mut list2;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            2 => b = i32::from_str(&arg).expect("Error parse."),
            3 => len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                if count < len {
                    list1_mut.next = Some(Box::new(ListNode::new(val)));
                    list1_mut = list1_mut.next.as_mut().unwrap();
                    count += 1;
                } else {
                    list2_mut.next = Some(Box::new(ListNode::new(val)));
                    list2_mut = list2_mut.next.as_mut().unwrap();
                }
            }
        }
    }

    if len == 0 || a > b || b >= len || len > ret {
        println!("Require at least (3 + arg3) parameters.");
        return;
    }

    let result: Option<Box<ListNode>> = Solution::merge_in_between(list1.next, a, b, list2.next);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
