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
    fn min_diff_in_bst_loop(node: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(x) = node {
            let v = x.borrow();
            Solution::min_diff_in_bst_loop(v.left.as_ref(), vec);
            vec.push(v.val);
            Solution::min_diff_in_bst_loop(v.right.as_ref(), vec);
        }
    }

    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        Solution::min_diff_in_bst_loop(root.as_ref(), &mut vec);
        let mut result: i32 = i32::MAX;
        for i in 1..vec.len() {
            let t = vec[i] - vec[i - 1];
            if t < result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut node = TreeNode::new(3);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!(
        "Minimum distance: {}",
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(node))))
    );
}
