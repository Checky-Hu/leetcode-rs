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
    fn invert_tree_loop(node: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = node {
            let val = x.borrow();
            let mut result: TreeNode = TreeNode::new(val.val);
            result.left = Solution::invert_tree_loop(val.right.as_ref());
            result.right = Solution::invert_tree_loop(val.left.as_ref());
            Some(Rc::new(RefCell::new(result)))
        } else {
            None
        }
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::invert_tree_loop(root.as_ref())
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    Solution::invert_tree(Some(Rc::new(RefCell::new(node))));
}
