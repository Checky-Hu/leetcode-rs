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
    fn count_nodes_loop(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = node {
            let val = x.borrow();
            1 + Solution::count_nodes_loop(val.left.as_ref())
                + Solution::count_nodes_loop(val.right.as_ref())
        } else {
            0
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::count_nodes_loop(root.as_ref())
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    println!(
        "Count nodes: {}",
        Solution::count_nodes(Some(Rc::new(RefCell::new(node))))
    );
}
