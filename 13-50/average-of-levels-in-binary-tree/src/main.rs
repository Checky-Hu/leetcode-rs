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
    fn average_of_levels_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        level: usize,
        result: &mut Vec<(f64, f64)>,
    ) {
        if let Some(x) = node {
            let v = x.borrow();
            if result.len() <= level {
                result.push((f64::from(v.val), 1_f64));
            } else {
                result[level].0 += f64::from(v.val);
                result[level].1 += 1_f64;
            }
            Solution::average_of_levels_loop(v.left.as_ref(), level + 1, result);
            Solution::average_of_levels_loop(v.right.as_ref(), level + 1, result);
        }
    }

    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut vec: Vec<(f64, f64)> = Vec::new();
        Solution::average_of_levels_loop(root.as_ref(), 0, &mut vec);
        let mut result: Vec<f64> = Vec::new();
        for v in vec.iter() {
            result.push(v.0 / v.1);
        }
        result
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let result = Solution::average_of_levels(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
