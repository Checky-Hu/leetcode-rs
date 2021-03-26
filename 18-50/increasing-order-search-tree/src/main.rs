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
    fn post_order_loop(node: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(x) = node {
            let v = x.borrow();
            Solution::post_order_loop(v.right.as_ref(), vec);
            vec.push(v.val);
            Solution::post_order_loop(v.left.as_ref(), vec);
        }
    }

    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vec: Vec<i32> = Vec::new();
        Solution::post_order_loop(root.as_ref(), &mut vec);
        let mut result: Option<Rc<RefCell<TreeNode>>> = None;
        for v in vec.iter() {
            let mut node: TreeNode = TreeNode::new(*v);
            node.right = result;
            result = Some(Rc::new(RefCell::new(node)));
        }
        result
    }
}

fn main() {
    let mut node = TreeNode::new(2);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    Solution::increasing_bst(Some(Rc::new(RefCell::new(node))));
}
