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
    fn is_unival_tree_loop(node: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(x) = node {
            let v = x.borrow();
            (val == -1 || v.val == val)
                && Solution::is_unival_tree_loop(v.left.as_ref(), v.val)
                && Solution::is_unival_tree_loop(v.right.as_ref(), v.val)
        } else {
            true
        }
    }

    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_unival_tree_loop(root.as_ref(), -1)
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Is univalued binary tree: {}",
        Solution::is_unival_tree(Some(Rc::new(RefCell::new(node))))
    );
}
