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
    fn convert_bst_loop(node: Option<&mut Rc<RefCell<TreeNode>>>, append: i32) -> i32 {
        if let Some(x) = node {
            let mut val = x.borrow_mut();
            let right = Solution::convert_bst_loop(val.right.as_mut(), append);
            val.val += right;
            let current = val.val;
            Solution::convert_bst_loop(val.left.as_mut(), current)
        } else {
            append
        }
    }

    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut result = root;
        Solution::convert_bst_loop(result.as_mut(), 0);
        result
    }
}

fn main() {
    let mut node = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    Solution::convert_bst(Some(Rc::new(RefCell::new(node))));
}
