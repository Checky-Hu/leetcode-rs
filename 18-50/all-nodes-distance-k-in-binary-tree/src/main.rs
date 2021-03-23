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
    fn distance_k_loop(
        node: Option<&Rc<RefCell<TreeNode>>>,
        father: i32,
        f_s_map: &mut HashMap<i32, Vec<i32>>,
        s_f_map: &mut HashMap<i32, i32>,
    ) {
        if let Some(x) = node {
            let v = x.borrow();
            match f_s_map.get_mut(&father) {
                Some(x) => x.push(v.val),
                None => {
                    f_s_map.insert(father, vec![v.val]);
                }
            }
            s_f_map.insert(v.val, father);
            Solution::distance_k_loop(v.left.as_ref(), v.val, f_s_map, s_f_map);
            Solution::distance_k_loop(v.right.as_ref(), v.val, f_s_map, s_f_map);
        }
    }

    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        K: i32,
    ) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if let Some(x) = p {
            let mut f_s_map: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut s_f_map: HashMap<i32, i32> = HashMap::new();
            Solution::distance_k_loop(root.as_ref(), -1, &mut f_s_map, &mut s_f_map);
            let v = x.borrow();
            result.push(v.val);
            let mut visits: Vec<bool> = vec![false; 501];
            visits[v.val as usize] = true;
            for _i in 0..K {
                let mut next: Vec<i32> = Vec::new();
                for r in result {
                    if let Some(x) = s_f_map.get(&r) {
                        if *x == -1 {
                            if let Some(y) = f_s_map.get(&r) {
                                for v in y.iter() {
                                    if !visits[*v as usize] {
                                        visits[*v as usize] = true;
                                        next.push(*v);
                                    }
                                }
                            }
                        } else if !visits[*x as usize] {
                            visits[*x as usize] = true;
                            next.push(*x);
                        }
                    }
                    if let Some(x) = f_s_map.get(&r) {
                        for v in x.iter() {
                            if !visits[*v as usize] {
                                visits[*v as usize] = true;
                                next.push(*v);
                            }
                        }
                    }
                }
                result = next;
            }
        }
        result
    }
}

fn main() {
    let mut node = TreeNode::new(3);
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let result = Solution::distance_k(
        Some(Rc::new(RefCell::new(node))),
        Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        2,
    );
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
