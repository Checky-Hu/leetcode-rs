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
    fn sorted_array_to_bst_loop(
        nums: &[i32],
        left: usize,
        right: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left == right {
            Some(Rc::new(RefCell::new(TreeNode::new(nums[left]))))
        } else if left + 1 == right {
            let mut node = TreeNode::new(nums[left]);
            node.right = Some(Rc::new(RefCell::new(TreeNode::new(nums[right]))));
            Some(Rc::new(RefCell::new(node)))
        } else {
            let mid: usize = left + (right - left) / 2;
            let mut node = TreeNode::new(nums[mid]);
            node.left = Solution::sorted_array_to_bst_loop(nums, left, mid - 1);
            node.right = Solution::sorted_array_to_bst_loop(nums, mid + 1, right);
            Some(Rc::new(RefCell::new(node)))
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len: usize = nums.len();
        if len == 0 {
            None
        } else {
            Solution::sorted_array_to_bst_loop(&nums, 0, len - 1)
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(num);
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    Solution::sorted_array_to_bst(nums);
}
