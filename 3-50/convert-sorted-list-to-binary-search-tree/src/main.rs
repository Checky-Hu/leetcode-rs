use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(v: i32) -> Self {
        ListNode { val: v, next: None }
    }
}

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
    fn sorted_array_to_bst_loop(nums: &[i32], left: usize, right: usize) -> Rc<RefCell<TreeNode>> {
        if left == right {
            Rc::new(RefCell::new(TreeNode::new(nums[left])))
        } else if left + 1 == right {
            let mut node = TreeNode::new(nums[left]);
            node.right = Some(Rc::new(RefCell::new(TreeNode::new(nums[right]))));
            Rc::new(RefCell::new(node))
        } else {
            let mid: usize = left + (right - left) / 2;
            let mut node = TreeNode::new(nums[mid]);
            node.left = Some(Solution::sorted_array_to_bst_loop(nums, left, mid - 1));
            node.right = Some(Solution::sorted_array_to_bst_loop(nums, mid + 1, right));
            Rc::new(RefCell::new(node))
        }
    }

    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pointer: Option<&Box<ListNode>> = head.as_ref();
        let mut nums: Vec<i32> = Vec::new();
        while let Some(x) = pointer {
            nums.push(x.val);
            pointer = x.next.as_ref();
        }
        let len: usize = nums.len();
        if len == 0 {
            None
        } else {
            Some(Solution::sorted_array_to_bst_loop(&nums, 0, len - 1))
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut head_mut: &mut Box<ListNode> = &mut head;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let val: i32 = i32::from_str(&arg).expect("Error parse.");
                head_mut.next = Some(Box::new(ListNode::new(val)));
                head_mut = head_mut.next.as_mut().unwrap();
            }
        }
    }

    if ret == 0 {
        println!("Require at least 1 parameter.");
        return;
    }

    Solution::sorted_list_to_bst(head.next);
}
