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
    fn find_ancestors_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        n: i32,
        ancestors: &mut Vec<i32>,
    ) -> bool {
        if let Some(x) = node {
            let val = x.borrow();
            if val.val == n {
                ancestors.push(val.val);
                true
            } else if Solution::find_ancestors_loop(val.left.as_ref(), n, ancestors)
                || Solution::find_ancestors_loop(val.right.as_ref(), n, ancestors)
            {
                ancestors.insert(0, val.val);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut p_ancestors: Vec<i32> = Vec::new();
        Solution::find_ancestors_loop(root.as_ref(), p.unwrap().borrow().val, &mut p_ancestors);
        let mut q_ancestors: Vec<i32> = Vec::new();
        Solution::find_ancestors_loop(root.as_ref(), q.unwrap().borrow().val, &mut q_ancestors);
        let p_len: usize = p_ancestors.len();
        let q_len: usize = q_ancestors.len();
        let mut i: usize = 0;
        while i < p_len && i < q_len {
            if p_ancestors[i] != q_ancestors[i] {
                break;
            } else {
                i += 1;
            }
        }
        Some(Rc::new(RefCell::new(TreeNode::new(p_ancestors[i - 1]))))
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    let result = Solution::lowest_common_ancestor(
        Some(Rc::new(RefCell::new(node))),
        Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    );
    println!("Lowest common ancestor: {}", result.unwrap().borrow().val);
}
