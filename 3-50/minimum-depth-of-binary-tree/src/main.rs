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
    fn min_depth_loop(node: &Rc<RefCell<TreeNode>>, depth: i32) -> i32 {
        let v = node.borrow();
        let tmp = depth + 1;
        if let Some(left) = &v.left {
            let result1 = Solution::min_depth_loop(&left, tmp);
            if let Some(right) = &v.right {
                let result2 = Solution::min_depth_loop(&right, tmp);
                if result1 < result2 {
                    result1
                } else {
                    result2
                }
            } else {
                result1
            }
        } else if let Some(right) = &v.right {
            Solution::min_depth_loop(&right, tmp)
        } else {
            tmp
        }
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = root {
            Solution::min_depth_loop(&x, 0)
        } else {
            0
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
        "Minimum depth: {}",
        Solution::min_depth(Some(Rc::new(RefCell::new(node))))
    );
}
