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
    fn sum_root_to_leaf_loop(node: &Rc<RefCell<TreeNode>>, val: i32) -> i32 {
        let v = node.borrow();
        let cur = (val << 1) + v.val;
        if let Some(x) = &v.left {
            Solution::sum_root_to_leaf_loop(&x, cur)
                + if let Some(y) = &v.right {
                    Solution::sum_root_to_leaf_loop(&y, cur)
                } else {
                    0
                }
        } else if let Some(x) = &v.right {
            Solution::sum_root_to_leaf_loop(&x, cur)
        } else {
            cur
        }
    }

    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = root {
            Solution::sum_root_to_leaf_loop(&x, 0)
        } else {
            0
        }
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    println!(
        "Sum of root to leaf binary numbers: {}",
        Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(node))))
    );
}
