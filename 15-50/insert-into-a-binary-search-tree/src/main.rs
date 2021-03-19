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
    fn insert_into_bst_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        val: i32,
        insert: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = node {
            let v = x.borrow();
            let mut node = TreeNode::new(v.val);
            if val > v.val {
                node.left = Solution::insert_into_bst_loop(v.left.as_ref(), val, false);
                node.right = Solution::insert_into_bst_loop(v.right.as_ref(), val, insert);
            } else {
                node.left = Solution::insert_into_bst_loop(v.left.as_ref(), val, insert);
                node.right = Solution::insert_into_bst_loop(v.right.as_ref(), val, false);
            }
            Some(Rc::new(RefCell::new(node)))
        } else if insert {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        } else {
            None
        }
    }

    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::insert_into_bst_loop(root.as_ref(), val, true)
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
                let mut node = TreeNode::new(3);
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
                Solution::insert_into_bst(Some(Rc::new(RefCell::new(node))), val);
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
