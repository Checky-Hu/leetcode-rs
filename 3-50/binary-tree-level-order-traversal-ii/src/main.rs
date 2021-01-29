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
    fn level_order_bottom_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        vec: &mut Vec<Vec<i32>>,
        level: usize,
    ) {
        if let Some(x) = node {
            if vec.len() <= level {
                vec.push(Vec::new());
            }
            let v = x.borrow();
            vec[level].push(v.val);
            Solution::level_order_bottom_loop(v.left.as_ref(), vec, level + 1);
            Solution::level_order_bottom_loop(v.right.as_ref(), vec, level + 1);
        }
    }

    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Solution::level_order_bottom_loop(root.as_ref(), &mut result, 0);
        result.reverse();
        result
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(1);
    node.left = left;
    node.right = right;
    let result = Solution::level_order_bottom(Some(Rc::new(RefCell::new(node))));
    for vec in result.iter() {
        for n in vec.iter() {
            print!("{} ", *n);
        }
        println!();
    }
}
