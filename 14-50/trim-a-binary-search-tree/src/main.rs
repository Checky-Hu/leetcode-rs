use std::cell::RefCell;
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
    fn trim_bst_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = node {
            let v = x.borrow();
            if v.val < low {
                Solution::trim_bst_loop(v.right.as_ref(), low, high)
            } else if v.val <= high {
                let mut node = TreeNode::new(v.val);
                node.left = Solution::trim_bst_loop(v.left.as_ref(), low, high);
                node.right = Solution::trim_bst_loop(v.right.as_ref(), low, high);
                Some(Rc::new(RefCell::new(node)))
            } else {
                Solution::trim_bst_loop(v.left.as_ref(), low, high)
            }
        } else {
            None
        }
    }

    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::trim_bst_loop(root.as_ref(), low, high)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut low: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => low = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let high: i32 = i32::from_str(&arg).expect("Error parse.");
                let mut node = TreeNode::new(2);
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
                Solution::trim_bst(Some(Rc::new(RefCell::new(node))), low, high);
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
