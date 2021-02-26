use std::cell::RefCell;
use std::cmp::Ordering;
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
    fn find_frequent_tree_sum_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, i32>,
        vec: &mut Vec<i32>,
        max: &mut i32,
    ) -> i32 {
        if let Some(x) = node {
            let val = x.borrow();
            let sum = val.val
                + Solution::find_frequent_tree_sum_loop(val.left.as_ref(), map, vec, max)
                + Solution::find_frequent_tree_sum_loop(val.right.as_ref(), map, vec, max);
            let tmp = match map.get_mut(&sum) {
                Some(x) => {
                    *x += 1;
                    *x
                }
                None => {
                    map.insert(sum, 1);
                    1
                }
            };
            match tmp.cmp(max) {
                Ordering::Less => (),
                Ordering::Equal => vec.push(sum),
                Ordering::Greater => {
                    vec.clear();
                    vec.push(sum);
                    *max = tmp;
                }
            }
            sum
        } else {
            0
        }
    }

    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut vec: Vec<i32> = Vec::new();
        let mut max: i32 = 0;
        Solution::find_frequent_tree_sum_loop(root.as_ref(), &mut map, &mut vec, &mut max);
        vec
    }
}

fn main() {
    let mut node = TreeNode::new(5);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    let result = Solution::find_frequent_tree_sum(Some(Rc::new(RefCell::new(node))));
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
