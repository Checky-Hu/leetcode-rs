use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    fn find_mode_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, i32>,
        result: &mut Vec<i32>,
        count: &mut i32,
    ) {
        if let Some(x) = node {
            let val = x.borrow();
            let t = match map.get_mut(&val.val) {
                Some(x) => {
                    *x += 1;
                    *x
                }
                None => {
                    map.insert(val.val, 1);
                    1
                }
            };
            match t.cmp(count) {
                Ordering::Less => (),
                Ordering::Equal => result.push(val.val),
                Ordering::Greater => {
                    result.clear();
                    result.push(val.val);
                    *count = t;
                }
            }
            Solution::find_mode_loop(val.left.as_ref(), map, result, count);
            Solution::find_mode_loop(val.right.as_ref(), map, result, count);
        }
    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();
        let mut count: i32 = 0;
        Solution::find_mode_loop(root.as_ref(), &mut map, &mut result, &mut count);
        result
    }
}

fn main() {
    let mut node = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let result = Solution::find_mode(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
