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
    fn construct_maximum_binary_tree_loop(
        nums: &[i32],
        left: usize,
        right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left >= right {
            None
        } else {
            let mut max: (usize, i32) = (left, nums[left]);
            for (i, v) in nums.iter().enumerate().take(right).skip(left + 1) {
                if *v > max.1 {
                    max.0 = i;
                    max.1 = *v;
                }
            }
            let mut node = TreeNode::new(max.1);
            node.left = Solution::construct_maximum_binary_tree_loop(nums, left, max.0);
            node.right = Solution::construct_maximum_binary_tree_loop(nums, max.0 + 1, right);
            Some(Rc::new(RefCell::new(node)))
        }
    }

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::construct_maximum_binary_tree_loop(&nums, 0, nums.len())
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    Solution::construct_maximum_binary_tree(nums);
}
