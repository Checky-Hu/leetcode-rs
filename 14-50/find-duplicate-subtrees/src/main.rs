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
    fn find_duplicate_subtrees_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, i32>,
        vec: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        let mut result: String = String::new();
        if let Some(x) = node {
            let v = x.borrow();
            result.push_str(&v.val.to_string());
            result.push(',');
            let left = Solution::find_duplicate_subtrees_loop(v.left.as_ref(), map, vec);
            result.push_str(&left);
            result.push(',');
            let right = Solution::find_duplicate_subtrees_loop(v.right.as_ref(), map, vec);
            result.push_str(&right);
            match map.get_mut(&result) {
                Some(y) => {
                    if *y == 1 {
                        vec.push(Some(x.clone()));
                    }
                    *y += 1;
                }
                None => {
                    map.insert(result.clone(), 1);
                }
            }
        } else {
            result.push('#');
        }
        result
    }

    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut vec: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        Solution::find_duplicate_subtrees_loop(root.as_ref(), &mut map, &mut vec);
        vec
    }
}

fn main() {
    let mut node = TreeNode::new(1);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    Solution::find_duplicate_subtrees(Some(Rc::new(RefCell::new(node))));
}
