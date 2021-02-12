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
    fn kth_smallest_loop(node: Option<&Rc<RefCell<TreeNode>>>, index: &mut i32) -> Option<i32> {
        if let Some(x) = node {
            let val = x.borrow();
            if let Some(y) = Solution::kth_smallest_loop(val.left.as_ref(), index) {
                Some(y)
            } else if *index == 1 {
                Some(val.val)
            } else {
                *index -= 1;
                Solution::kth_smallest_loop(val.right.as_ref(), index)
            }
        } else {
            None
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut index: i32 = k;
        if let Some(x) = Solution::kth_smallest_loop(root.as_ref(), &mut index) {
            x
        } else {
            0
        }
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    println!(
        "2nd samllest element: {}",
        Solution::kth_smallest(Some(Rc::new(RefCell::new(node))), 2)
    );
}
