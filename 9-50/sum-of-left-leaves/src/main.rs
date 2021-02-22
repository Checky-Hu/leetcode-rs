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
    fn sum_of_left_leaves_loop(node: Option<&Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(x) = node {
            let v = x.borrow();
            if v.left == None && v.right == None {
                if is_left {
                    v.val
                } else {
                    0
                }
            } else {
                Solution::sum_of_left_leaves_loop(v.left.as_ref(), true)
                    + Solution::sum_of_left_leaves_loop(v.right.as_ref(), false)
            }
        } else {
            0
        }
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::sum_of_left_leaves_loop(root.as_ref(), false)
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    println!(
        "Sum of left leaves: {}",
        Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(node))))
    );
}
