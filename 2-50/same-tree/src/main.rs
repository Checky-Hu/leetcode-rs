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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(x) = p {
            if let Some(y) = q {
                let a = x.borrow();
                let b = y.borrow();
                a.val == b.val
                    && Solution::is_same_tree(a.left.clone(), b.left.clone())
                    && Solution::is_same_tree(a.right.clone(), b.right.clone())
            } else {
                false
            }
        } else {
            None == q
        }
    }
}

fn main() {
    let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("Same tree: {}", Solution::is_same_tree(p, q));
}
