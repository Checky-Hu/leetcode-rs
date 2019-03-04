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
        ListNode {
	    val: v,
	    next: None,
	}
    }
}

struct Solution {
}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        l1
    }
}

fn main() {
    let mut strs: [String; 2] = ["".to_string(), "".to_string()];
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            strs[0] = arg;
	} else if index == 2 {
            strs[1] = arg;
	    break;
	} else {
	    continue;
	}
    }

    if 0 == strs[1].len() {
        println!("Require at least two parameter.");
	return;
    }

    let mut vecs: [Vec<&str>; 2]= [vec!(), vec!()];
    let mut index: usize = 0;
    let mut ls1: Option<Box<ListNode>> = None;
    let mut ls2: Option<Box<ListNode>> = None;
    while index < vecs.len() {
        vecs[index] = strs[index].split(',').collect();

	let mut tmp_i: usize = 0;
	let mut tmp_list: &Option<Box<ListNode>> = match index {
	    0 => &mut ls1,
	    1 => &mut ls2,
	    _ => &None,
	};
        for s in &vecs[index] {
	    let tmp_node = ListNode::new(i32::from_str(&s).expect("Parse Error"));
	    match tmp_i {
	        0 => *tmp_list = Some(Box::new(tmp_node)),
		_ => {
		    match tmp_list {
		        Some(ref p) => {
			    p.next = Some(Box::new(tmp_node));
			    *tmp_list = p.next;
			},
		        None => (),
		    }
		},
	    }
	    tmp_i += 1;
	}
	index += 1;
    }

    let mut result = Solution::merge_two_lists(ls1, ls2);
    print!("List: ");
    while result != None {
        print!("{} ", result.unwrap().val);
	result = result.unwrap().next;
    }
    print!("\n");
}
