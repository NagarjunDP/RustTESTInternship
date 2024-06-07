
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
   
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(right_left)));
    root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(right_right)));

    println!("Maximum depth of the tree: {}", max_depth(Some(Rc::new(RefCell::new(root)))));
}
