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
    fn prune_tree_loop(node: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = node {
            let v = x.borrow();
            let left = Solution::prune_tree_loop(v.left.as_ref());
            let right = Solution::prune_tree_loop(v.right.as_ref());
            if v.val == 0 && left == None && right == None {
                None
            } else {
                let mut node = TreeNode::new(v.val);
                node.left = left;
                node.right = right;
                Some(Rc::new(RefCell::new(node)))
            }
        } else {
            None
        }
    }

    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::prune_tree_loop(root.as_ref())
    }
}

fn main() {
    let mut node = TreeNode::new(0);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    Solution::prune_tree(Some(Rc::new(RefCell::new(node))));
}
