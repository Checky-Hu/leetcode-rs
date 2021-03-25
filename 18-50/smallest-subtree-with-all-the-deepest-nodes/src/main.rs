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
    fn count_levels_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        level: usize,
        result: &mut Vec<i32>,
    ) {
        if let Some(x) = node {
            let v = x.borrow();
            if result.len() == level {
                result.push(1);
            } else {
                result[level] += 1;
            }
            Solution::count_levels_loop(v.left.as_ref(), level + 1, result);
            Solution::count_levels_loop(v.right.as_ref(), level + 1, result);
        }
    }

    fn subtree_with_all_deepest_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        level: usize,
        target: (usize, i32),
    ) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(x) = node {
            let v = x.borrow();
            if level == target.0 {
                (1, Some(Rc::new(RefCell::new(TreeNode::new(v.val)))))
            } else {
                let left =
                    Solution::subtree_with_all_deepest_loop(v.left.as_ref(), level + 1, target);
                let right =
                    Solution::subtree_with_all_deepest_loop(v.right.as_ref(), level + 1, target);
                if left.0 == target.1 {
                    left
                } else if right.0 == target.1 {
                    right
                } else {
                    let mut node = TreeNode::new(v.val);
                    node.left = left.1;
                    node.right = right.1;
                    (left.0 + right.0, Some(Rc::new(RefCell::new(node))))
                }
            }
        } else {
            (0, None)
        }
    }

    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut counts: Vec<i32> = Vec::new();
        Solution::count_levels_loop(root.as_ref(), 0, &mut counts);
        if counts.is_empty() {
            None
        } else {
            let level = counts.len() - 1;
            let result =
                Solution::subtree_with_all_deepest_loop(root.as_ref(), 0, (level, counts[level]));
            result.1
        }
    }
}

fn main() {
    let mut node = TreeNode::new(0);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    Solution::subtree_with_all_deepest(Some(Rc::new(RefCell::new(node))));
}
