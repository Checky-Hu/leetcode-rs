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
    fn binary_tree_paths_loop(
        node: &Rc<RefCell<TreeNode>>,
        results: &mut Vec<String>,
        current: &mut String,
    ) {
        let v = node.borrow();
        let s = v.val.to_string();
        let mut len: usize = s.len();
        current.push_str(&s);
        if let Some(left) = &v.left {
            current.push('-');
            current.push('>');
            len += 2;
            Solution::binary_tree_paths_loop(&left, results, current);
            if let Some(right) = &v.right {
                Solution::binary_tree_paths_loop(&right, results, current);
            }
        } else if let Some(right) = &v.right {
            current.push('-');
            current.push('>');
            len += 2;
            Solution::binary_tree_paths_loop(&right, results, current);
        } else {
            results.push(current.clone());
        }
        for _i in 0..len {
            current.pop();
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let mut current: String = String::new();
        if let Some(x) = root {
            Solution::binary_tree_paths_loop(&x, &mut results, &mut current);
        }
        results
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    let result = Solution::binary_tree_paths(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        println!("{}", *r);
    }
}
