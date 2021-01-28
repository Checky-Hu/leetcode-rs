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
    fn is_balanced_loop(node: Option<&Rc<RefCell<TreeNode>>>, depth: i32) -> (i32, bool) {
        if let Some(x) = node {
            let val = x.borrow();
            let tmp = depth + 1;
            let result1 = Solution::is_balanced_loop(val.left.as_ref(), tmp);
            if !result1.1 {
                return (0, false);
            }
            let result2 = Solution::is_balanced_loop(val.right.as_ref(), tmp);
            if !result2.1 {
                return (0, false);
            }
            if result1.0 > result2.0 {
                (result1.0, result1.0 - result2.0 < 2)
            } else {
                (result2.0, result2.0 - result1.0 < 2)
            }
        } else {
            (depth, true)
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let result = Solution::is_balanced_loop(root.as_ref(), 0);
        result.1
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = None;
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    println!(
        "Is balanced: {}",
        Solution::is_balanced(Some(Rc::new(RefCell::new(node))))
    );
}
