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
    fn is_even_odd_tree_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
        level: usize,
    ) -> bool {
        if let Some(x) = node {
            let val = x.borrow();
            if (level as i32 & 1) ^ (val.val & 1) == 0 {
                return false;
            }
            if result.len() == level {
                result.push(val.val);
            } else if level & 1 == 1 {
                if val.val >= result[level] {
                    return false;
                } else {
                    result[level] = val.val;
                }
            } else if val.val <= result[level] {
                return false;
            } else {
                result[level] = val.val;
            }
            Solution::is_even_odd_tree_loop(val.left.as_ref(), result, level + 1)
                && Solution::is_even_odd_tree_loop(val.right.as_ref(), result, level + 1)
        } else {
            true
        }
    }

    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result: Vec<i32> = Vec::new();
        Solution::is_even_odd_tree_loop(root.as_ref(), &mut result, 0)
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!(
        "Is even odd tree: {}",
        Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(node))))
    );
}
