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
    fn right_side_view_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        vec: &mut Vec<i32>,
        level: usize,
    ) {
        if let Some(x) = node {
            let v = x.borrow();
            if vec.len() <= level {
                vec.push(v.val);
            } else {
                vec[level] = v.val;
            }
            Solution::right_side_view_loop(v.left.as_ref(), vec, level + 1);
            Solution::right_side_view_loop(v.right.as_ref(), vec, level + 1);
        }
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Solution::right_side_view_loop(root.as_ref(), &mut result, 0);
        result
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = None; // Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    let result = Solution::right_side_view(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
