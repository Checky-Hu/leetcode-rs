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
    fn has_path_sum_loop(node: &Rc<RefCell<TreeNode>>, cur: i32, sum: i32) -> bool {
        let v = node.borrow();
        let tmp = cur + v.val;
        if let Some(left) = &v.left {
            if Solution::has_path_sum_loop(&left, tmp, sum) {
                return true;
            }
            if let Some(right) = &v.right {
                Solution::has_path_sum_loop(&right, tmp, sum)
            } else {
                false
            }
        } else if let Some(right) = &v.right {
            Solution::has_path_sum_loop(&right, tmp, sum)
        } else {
            tmp == sum
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(x) = root {
            Solution::has_path_sum_loop(&x, 0, sum)
        } else {
            sum == 0
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
        "Has path sum: {}",
        Solution::has_path_sum(Some(Rc::new(RefCell::new(node))), 4)
    );
}
