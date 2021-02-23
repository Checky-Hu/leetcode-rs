use std::cell::RefCell;
use std::collections::HashMap;
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
    fn path_sum_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, i32>,
        cur: i32,
        sum: i32,
    ) -> i32 {
        if let Some(x) = node {
            let v = x.borrow();
            let tmp = cur + v.val;
            let result = match map.get(&(tmp - sum)) {
                Some(y) => *y,
                None => 0,
            };
            match map.get_mut(&tmp) {
                Some(y) => *y += 1,
                None => {
                    map.insert(tmp, 1);
                }
            }
            let left = Solution::path_sum_loop(v.left.as_ref(), map, tmp, sum);
            let right = Solution::path_sum_loop(v.right.as_ref(), map, tmp, sum);
            if let Some(y) = map.get_mut(&tmp) {
                *y -= 1;
            }
            result + left + right
        } else {
            0
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        Solution::path_sum_loop(root.as_ref(), &mut map, 0, sum)
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(-2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    let mut node = TreeNode::new(1);
    node.left = left;
    node.right = right;
    println!(
        "Path sum: {}",
        Solution::path_sum(Some(Rc::new(RefCell::new(node))), -1)
    );
}
