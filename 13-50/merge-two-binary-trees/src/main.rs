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
    fn merge_trees_loop(
        node1: Option<&Rc<RefCell<TreeNode>>>,
        node2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = node1 {
            let v1 = x.borrow();
            if let Some(y) = node2 {
                let v2 = y.borrow();
                let mut result = TreeNode::new(v1.val + v2.val);
                result.left = Solution::merge_trees_loop(v1.left.as_ref(), v2.left.as_ref());
                result.right = Solution::merge_trees_loop(v1.right.as_ref(), v2.right.as_ref());
                Some(Rc::new(RefCell::new(result)))
            } else {
                let mut result = TreeNode::new(v1.val);
                result.left = Solution::merge_trees_loop(v1.left.as_ref(), None);
                result.right = Solution::merge_trees_loop(v1.right.as_ref(), None);
                Some(Rc::new(RefCell::new(result)))
            }
        } else if let Some(x) = node2 {
            let v = x.borrow();
            let mut result = TreeNode::new(v.val);
            result.left = Solution::merge_trees_loop(None, v.left.as_ref());
            result.right = Solution::merge_trees_loop(None, v.right.as_ref());
            Some(Rc::new(RefCell::new(result)))
        } else {
            None
        }
    }

    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::merge_trees_loop(root1.as_ref(), root2.as_ref())
    }
}

fn main() {
    let root1 = TreeNode::new(1);
    let mut root2 = TreeNode::new(1);
    root2.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root2.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    Solution::merge_trees(
        Some(Rc::new(RefCell::new(root1))),
        Some(Rc::new(RefCell::new(root2))),
    );
}
