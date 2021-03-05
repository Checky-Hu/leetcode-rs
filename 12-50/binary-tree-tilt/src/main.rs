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
    fn find_tilt_loop(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(x) = node {
            let val = x.borrow();
            let left = Solution::find_tilt_loop(val.left.as_ref());
            let right = Solution::find_tilt_loop(val.right.as_ref());
            (
                left.0 + right.0 + val.val,
                left.1 + right.1 + (left.0 - right.0).abs(),
            )
        } else {
            (0, 0)
        }
    }

    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // (sum, tilt)
        let result = Solution::find_tilt_loop(root.as_ref());
        result.1
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Tilt: {}",
        Solution::find_tilt(Some(Rc::new(RefCell::new(node))))
    );
}
