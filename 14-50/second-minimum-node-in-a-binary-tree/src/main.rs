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
    fn find_second_minimum_value_loop(node: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        if let Some(x) = node {
            let v = x.borrow();
            if val == 0 || val == v.val {
                let left = Solution::find_second_minimum_value_loop(v.left.as_ref(), v.val);
                let right = Solution::find_second_minimum_value_loop(v.right.as_ref(), v.val);
                if left == -1 {
                    if right == -1 {
                        -1
                    } else {
                        right
                    }
                } else if right == -1 || left < right {
                    left
                } else {
                    right
                }
            } else {
                v.val
            }
        } else {
            -1
        }
    }

    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::find_second_minimum_value_loop(root.as_ref(), 0)
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Second minimum value: {}",
        Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(node))))
    );
}
