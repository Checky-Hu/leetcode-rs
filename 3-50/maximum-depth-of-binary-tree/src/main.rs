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
    fn max_depth_loop(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = node {
            let a = x.borrow();
            let t1 = Solution::max_depth_loop(a.left.as_ref());
            let t2 = Solution::max_depth_loop(a.right.as_ref());
            (if t1 > t2 { t1 } else { t2 }) + 1
        } else {
            0
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_loop(root.as_ref())
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = None;
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    println!(
        "Maximum depth of binary tree: {}",
        Solution::max_depth(Some(Rc::new(RefCell::new(node))))
    );
}
