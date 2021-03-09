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
    fn add_one_row_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
        level: i32,
        is_left: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if level == d {
            let mut result = TreeNode::new(v);
            if is_left {
                result.left = Solution::add_one_row_loop(node, v, d, level + 1, true);
            } else {
                result.right = Solution::add_one_row_loop(node, v, d, level + 1, false);
            }
            Some(Rc::new(RefCell::new(result)))
        } else if let Some(x) = node {
            let next = x.borrow();
            let mut result = TreeNode::new(next.val);
            result.left = Solution::add_one_row_loop(next.left.as_ref(), v, d, level + 1, true);
            result.right = Solution::add_one_row_loop(next.right.as_ref(), v, d, level + 1, false);
            Some(Rc::new(RefCell::new(result)))
        } else {
            None
        }
    }

    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::add_one_row_loop(root.as_ref(), v, d, 1, true)
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    Solution::add_one_row(Some(Rc::new(RefCell::new(node))), 2, 1);
}
