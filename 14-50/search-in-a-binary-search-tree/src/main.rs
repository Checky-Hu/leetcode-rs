use std::cell::RefCell;
use std::cmp::Ordering;
use std::env;
use std::rc::Rc;
use std::str::FromStr;

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
    fn search_bst_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        val: i32,
        found: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = node {
            let v = x.borrow();
            if found {
                let mut node = TreeNode::new(v.val);
                node.left = Solution::search_bst_loop(v.left.as_ref(), val, true);
                node.right = Solution::search_bst_loop(v.right.as_ref(), val, true);
                Some(Rc::new(RefCell::new(node)))
            } else {
                match val.cmp(&v.val) {
                    Ordering::Less => Solution::search_bst_loop(v.left.as_ref(), val, false),
                    Ordering::Equal => {
                        let mut node = TreeNode::new(v.val);
                        node.left = Solution::search_bst_loop(v.left.as_ref(), val, true);
                        node.right = Solution::search_bst_loop(v.right.as_ref(), val, true);
                        Some(Rc::new(RefCell::new(node)))
                    }
                    Ordering::Greater => Solution::search_bst_loop(v.right.as_ref(), val, false),
                }
            }
        } else {
            None
        }
    }

    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::search_bst_loop(root.as_ref(), val, false)
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                let mut node = TreeNode::new(2);
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
                Solution::search_bst(Some(Rc::new(RefCell::new(node))), val);
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
