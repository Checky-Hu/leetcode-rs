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
    fn preorder_traversal_loop(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(x) = node {
            let val = x.borrow();
            result.push(val.val);
            Solution::preorder_traversal_loop(val.left.as_ref(), result);
            Solution::preorder_traversal_loop(val.right.as_ref(), result);
        }
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut result: Vec<i32> = Vec::new();
        Solution::preorder_traversal_loop(root.as_ref(), &mut result);
        let len: usize = result.len();
        if len > 0 {
            let mut index: usize = len - 1;
            let mut node: Option<Rc<RefCell<TreeNode>>> = None;
            loop {
                let mut temp: TreeNode = TreeNode::new(result[index]);
                temp.right = node;
                node = Some(Rc::new(RefCell::new(temp)));
                if index == 0 {
                    break;
                } else {
                    index -= 1;
                }
            }
            *root = node;
        }
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = None;
    let mut node = TreeNode::new(1);
    node.left = left;
    node.right = right;
    Solution::flatten(&mut Some(Rc::new(RefCell::new(node))));
}
