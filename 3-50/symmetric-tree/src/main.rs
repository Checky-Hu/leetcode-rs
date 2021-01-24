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
    fn is_symmetric_loop(
        left: Option<&Rc<RefCell<TreeNode>>>,
        right: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(x) = left {
            let a = x.borrow();
            if let Some(y) = right {
                let b = y.borrow();
                a.val == b.val
                    && Solution::is_symmetric_loop(a.left.as_ref(), b.right.as_ref())
                    && Solution::is_symmetric_loop(a.right.as_ref(), b.left.as_ref())
            } else {
                false
            }
        } else {
            None == right
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(x) = root {
            let a = x.borrow();
            Solution::is_symmetric_loop(a.left.as_ref(), a.right.as_ref())
        } else {
            true
        }
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = None;
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    println!(
        "Symmetric tree: {}",
        Solution::is_symmetric(Some(Rc::new(RefCell::new(node))))
    );
}
