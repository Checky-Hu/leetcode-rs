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
    fn longest_univalue_path_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        value: i32,
        result: &mut i32,
    ) -> i32 {
        if let Some(x) = node {
            let v = x.borrow();
            let left = Solution::longest_univalue_path_loop(v.left.as_ref(), v.val, result);
            let right = Solution::longest_univalue_path_loop(v.right.as_ref(), v.val, result);
            let current = left + right;
            if current > *result {
                *result = current;
            }
            if v.val == value {
                1 + if left > right { left } else { right }
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: i32 = 0;
        Solution::longest_univalue_path_loop(root.as_ref(), 1001, &mut result);
        result
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!(
        "Longest univalue path: {}",
        Solution::longest_univalue_path(Some(Rc::new(RefCell::new(node))))
    );
}
