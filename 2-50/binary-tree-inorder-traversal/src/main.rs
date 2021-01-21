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
    fn inorder_traversal_loop(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(x) = node {
            let a = x.borrow();
            Solution::inorder_traversal_loop(a.left.clone(), vec);
            vec.push(a.val);
            Solution::inorder_traversal_loop(a.right.clone(), vec);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Solution::inorder_traversal_loop(root, &mut result);
        result
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(1);
    node.left = left;
    node.right = right;
    let result = Solution::inorder_traversal(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
