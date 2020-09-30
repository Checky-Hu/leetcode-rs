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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists_mut: Vec<Option<Box<ListNode>>> = lists;
        let len: usize = lists_mut.len();
        let mut header: Box<ListNode> = Box::new(ListNode::new(0));
        let mut result: &mut Box<ListNode> = &mut header;
        loop {
            let mut has_next: bool = false;
            let mut min: (i32, usize) = (i32::max_value(), len);
            for (i, v) in lists_mut.iter().enumerate() {
                if let Some(x) = v {
                    has_next = true;
                    if x.val < min.0 {
                        min.0 = x.val;
                        min.1 = i;
                    }
                }
            }
            if !has_next {
                break;
            } else {
                result.next = lists_mut[min.1].take();
                result = result.next.as_mut().unwrap();
                lists_mut[min.1] = result.next.take();
            }
        }
        header.next
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: i32 = -1;
    let mut count: i32 = 0;
    let mut list: Box<ListNode> = Box::new(ListNode::new(0));
    let mut list_mut: &mut Box<ListNode> = &mut list;
    let mut vec: Vec<Box<ListNode>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                if len == -1 {
                    len = i32::from_str(&arg).expect("Error parse.");
                } else {
                    let val: i32 = i32::from_str(&arg).expect("Error parse.");
                    list_mut.next = Some(Box::new(ListNode::new(val)));
                    list_mut = list_mut.next.as_mut().unwrap();
                    count += 1;
                    if count == len {
                        len = -1;
                        count = 0;
                        vec.push(list);
                        list = Box::new(ListNode::new(0));
                        list_mut = &mut list;
                    }
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least 1 parameter.");
        return;
    }

    let mut lists: Vec<Option<Box<ListNode>>> = Vec::new();
    for v in vec {
        lists.push(v.next);
    }
    let result: Option<Box<ListNode>> = Solution::merge_k_lists(lists);
    let mut t: Option<&Box<ListNode>> = result.as_ref();
    while let Some(x) = t {
        print!("{} ", x.val);
        t = x.next.as_ref();
    }
    println!();
}
