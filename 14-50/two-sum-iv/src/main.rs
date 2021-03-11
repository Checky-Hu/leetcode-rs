use std::cell::RefCell;
use std::collections::HashSet;
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
    fn find_target_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        k: i32,
        set: &mut HashSet<i32>,
    ) -> bool {
        if let Some(x) = node {
            let v = x.borrow();
            if set.contains(&(k - v.val)) {
                true
            } else {
                set.insert(v.val);
                Solution::find_target_loop(v.left.as_ref(), k, set)
                    || Solution::find_target_loop(v.right.as_ref(), k, set)
            }
        } else {
            false
        }
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        Solution::find_target_loop(root.as_ref(), k, &mut set)
    }
}

fn main() {
    let k: i32 = 5;
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!(
        "Two sum equal {}: {}",
        k,
        Solution::find_target(Some(Rc::new(RefCell::new(node))), k)
    );
}
