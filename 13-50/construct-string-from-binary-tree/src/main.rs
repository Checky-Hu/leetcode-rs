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
    fn tree2str_loop(node: &Rc<RefCell<TreeNode>>, result: &mut String) {
        let v = node.borrow();
        result.push_str(&v.val.to_string());
        if let Some(x) = &v.left {
            result.push('(');
            Solution::tree2str_loop(&x, result);
            result.push(')');
            if let Some(y) = &v.right {
                result.push('(');
                Solution::tree2str_loop(&y, result);
                result.push(')');
            }
        } else if let Some(x) = &v.right {
            result.push('(');
            result.push(')');
            result.push('(');
            Solution::tree2str_loop(&x, result);
            result.push(')');
        }
    }

    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result: String = String::new();
        if let Some(x) = t {
            Solution::tree2str_loop(&x, &mut result);
        }
        result
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Tree to string: {}",
        Solution::tree2str(Some(Rc::new(RefCell::new(node))))
    );
}
