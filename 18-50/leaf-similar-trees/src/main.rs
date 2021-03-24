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
    fn leaf_similar_loop(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) -> bool {
        if let Some(x) = node {
            let v = x.borrow();
            let left = Solution::leaf_similar_loop(v.left.as_ref(), result);
            let right = Solution::leaf_similar_loop(v.right.as_ref(), result);
            if left && right {
                result.push(v.val);
            }
            false
        } else {
            true
        }
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut result1: Vec<i32> = Vec::new();
        Solution::leaf_similar_loop(root1.as_ref(), &mut result1);
        let mut result2: Vec<i32> = Vec::new();
        Solution::leaf_similar_loop(root2.as_ref(), &mut result2);
        result1 == result2
    }
}

fn main() {
    let mut node1 = TreeNode::new(3);
    node1.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node1.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let mut node2 = TreeNode::new(1);
    node2.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node2.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!(
        "Leaf similar trees: {}",
        Solution::leaf_similar(
            Some(Rc::new(RefCell::new(node1))),
            Some(Rc::new(RefCell::new(node2)))
        )
    );
}
