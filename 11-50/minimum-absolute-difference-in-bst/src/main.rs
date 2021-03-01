use std::cell::RefCell;
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
    fn get_minimum_difference_loop(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(x) = node {
            let val = x.borrow();
            Solution::get_minimum_difference_loop(val.left.as_ref(), result);
            result.push(val.val);
            Solution::get_minimum_difference_loop(val.right.as_ref(), result);
        }
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        Solution::get_minimum_difference_loop(root.as_ref(), &mut vec);
        let mut result: i32 = i32::MAX;
        let mut prefix: i32 = vec[0];
        for v in vec.iter().skip(1) {
            let t: i32 = *v - prefix;
            if t < result {
                result = t;
            }
            prefix = *v;
        }
        result
    }
}

fn main() {
    let mut node = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Minimum absolute difference: {}",
        Solution::get_minimum_difference(Some(Rc::new(RefCell::new(node))))
    );
}
