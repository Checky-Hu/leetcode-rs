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
    fn postorder_traversal_loop(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(x) = node {
            let val = x.borrow();
            Solution::postorder_traversal_loop(val.left.as_ref(), result);
            Solution::postorder_traversal_loop(val.right.as_ref(), result);
            result.push(val.val);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Solution::postorder_traversal_loop(root.as_ref(), &mut result);
        result
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = None;
    let mut node = TreeNode::new(1);
    node.left = left;
    node.right = right;
    let result = Solution::postorder_traversal(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
