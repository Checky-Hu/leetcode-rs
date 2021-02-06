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
    fn sum_numbers_loop(node: &Rc<RefCell<TreeNode>>, cur: i32) -> i32 {
        let v = node.borrow();
        let tmp = cur * 10 + v.val;
        if let Some(left) = &v.left {
            let result1 = Solution::sum_numbers_loop(&left, tmp);
            if let Some(right) = &v.right {
                result1 + Solution::sum_numbers_loop(&right, tmp)
            } else {
                result1
            }
        } else if let Some(right) = &v.right {
            Solution::sum_numbers_loop(&right, tmp)
        } else {
            tmp
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = root {
            Solution::sum_numbers_loop(&x, 0)
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
        "Sum numbers: {}",
        Solution::sum_numbers(Some(Rc::new(RefCell::new(node))))
    );
}
