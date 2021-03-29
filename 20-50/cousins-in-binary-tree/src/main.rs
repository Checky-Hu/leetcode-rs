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
    fn get_info_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        parent: (i32, i32),
        val: i32,
        info: &mut (i32, i32),
    ) -> bool {
        if let Some(x) = node {
            let v = x.borrow();
            if v.val == val {
                info.0 = parent.0 + 1;
                info.1 = parent.1;
                true
            } else if !Solution::get_info_loop(v.left.as_ref(), (parent.0 + 1, v.val), val, info) {
                Solution::get_info_loop(v.right.as_ref(), (parent.0 + 1, v.val), val, info)
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        // (level, val)
        let mut info_x: (i32, i32) = (-1, 0);
        Solution::get_info_loop(root.as_ref(), (-1, 0), x, &mut info_x);
        let mut info_y: (i32, i32) = (-1, 0);
        Solution::get_info_loop(root.as_ref(), (-1, 0), y, &mut info_y);
        info_x.0 == info_y.0 && info_x.1 != info_y.1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => x = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let y: i32 = i32::from_str(&arg).expect("Error parse.");
                let mut node = TreeNode::new(2);
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
                println!(
                    "Is cousins: {}",
                    Solution::is_cousins(Some(Rc::new(RefCell::new(node))), x, y)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
