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

struct BSTIterator {
    array_: Vec<i32>,
    index_: usize,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut result: Vec<i32> = Vec::new();
        BSTIterator::preorder_traversal_loop(root.as_ref(), &mut result);
        BSTIterator {
            array_: result,
            index_: 0,
        }
    }

    fn preorder_traversal_loop(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(x) = node {
            let val = x.borrow();
            BSTIterator::preorder_traversal_loop(val.left.as_ref(), result);
            result.push(val.val);
            BSTIterator::preorder_traversal_loop(val.right.as_ref(), result);
        }
    }

    fn next(&mut self) -> i32 {
        self.index_ += 1;
        self.array_[self.index_ - 1]
    }

    fn has_next(&self) -> bool {
        self.index_ < self.array_.len()
    }
}

fn main() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node = TreeNode::new(2);
    node.left = left;
    node.right = right;
    let mut obj = BSTIterator::new(Some(Rc::new(RefCell::new(node))));
    println!("Next: {}", obj.next());
    println!("Has next: {}", obj.has_next());
    println!("Next: {}", obj.next());
    println!("Has next: {}", obj.has_next());
    println!("Next: {}", obj.next());
}
