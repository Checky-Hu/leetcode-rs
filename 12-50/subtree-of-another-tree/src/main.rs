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
    fn is_subtree_loop(
        s: Option<&Rc<RefCell<TreeNode>>>,
        t: Option<&Rc<RefCell<TreeNode>>>,
        is_root: bool,
    ) -> bool {
        if let Some(x) = s {
            if let Some(y) = t {
                let v1 = x.borrow();
                let v2 = y.borrow();
                if v1.val == v2.val
                    && Solution::is_subtree_loop(v1.left.as_ref(), v2.left.as_ref(), false)
                    && Solution::is_subtree_loop(v1.right.as_ref(), v2.right.as_ref(), false)
                {
                    return true;
                }
                is_root
                    && (Solution::is_subtree_loop(v1.left.as_ref(), t, true)
                        || Solution::is_subtree_loop(v1.right.as_ref(), t, true))
            } else {
                false
            }
        } else {
            None == t
        }
    }

    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_subtree_loop(s.as_ref(), t.as_ref(), true)
    }
}

fn main() {
    let mut s = TreeNode::new(1);
    s.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    s.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Is subtree: {}",
        Solution::is_subtree(
            Some(Rc::new(RefCell::new(s))),
            Some(Rc::new(RefCell::new(TreeNode::new(2))))
        )
    );
}
