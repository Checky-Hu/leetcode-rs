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
    fn find_bottom_left_value_loop(node: Option<&Rc<RefCell<TreeNode>>>, level: i32) -> (i32, i32) {
        if let Some(x) = node {
            let val = x.borrow();
            let left = Solution::find_bottom_left_value_loop(val.left.as_ref(), level + 1);
            let right = Solution::find_bottom_left_value_loop(val.right.as_ref(), level + 1);
            if left.0 != 0 {
                if right.0 != 0 {
                    if right.0 > left.0 {
                        right
                    } else {
                        left
                    }
                } else {
                    left
                }
            } else if right.0 != 0 {
                right
            } else {
                (level, val.val)
            }
        } else {
            (0, 0)
        }
    }

    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Solution::find_bottom_left_value_loop(root.as_ref(), 0);
        result.1
    }
}

fn main() {
    let mut node = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Bottom left tree value: {}",
        Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(node))))
    );
}
