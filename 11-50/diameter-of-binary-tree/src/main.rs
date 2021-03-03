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
    fn diameter_of_binary_tree_loop(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if let Some(x) = node {
            let val = x.borrow();
            let left = Solution::diameter_of_binary_tree_loop(val.left.as_ref(), result);
            let right = Solution::diameter_of_binary_tree_loop(val.right.as_ref(), result);
            let sum = left + right;
            if sum > *result {
                *result = sum;
            }
            1 + if left > right { left } else { right }
        } else {
            0
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: i32 = 0;
        Solution::diameter_of_binary_tree_loop(root.as_ref(), &mut result);
        result
    }
}

fn main() {
    let mut node = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Diameter of binary tree: {}",
        Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(node))))
    );
}
