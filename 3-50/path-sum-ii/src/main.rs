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
    fn path_sum_loop(
        node: &Rc<RefCell<TreeNode>>,
        cur: i32,
        sum: i32,
        results: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>,
    ) {
        let v = node.borrow();
        let tmp = cur + v.val;
        current.push(v.val);
        if let Some(left) = &v.left {
            Solution::path_sum_loop(&left, tmp, sum, results, current);
            if let Some(right) = &v.right {
                Solution::path_sum_loop(&right, tmp, sum, results, current);
            }
        } else if let Some(right) = &v.right {
            Solution::path_sum_loop(&right, tmp, sum, results, current);
        } else if tmp == sum {
            results.push(current.clone());
        }
        current.pop();
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();
        if let Some(x) = root {
            Solution::path_sum_loop(&x, 0, target_sum, &mut results, &mut current);
        }
        results
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = None;
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    let result = Solution::path_sum(Some(Rc::new(RefCell::new(node))), 4);
    for r in result.iter() {
        for v in r.iter() {
            print!("{} ", *v);
        }
        println!();
    }
}
