use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref r) = root {
        let left = r.borrow_mut().left.clone(); // Clone before borrowing mutably
        let right = r.borrow_mut().right.clone(); // Clone before borrowing mutably
        r.borrow_mut().left = invert_tree(right); // Pass the cloned right to invert_tree
        r.borrow_mut().right = invert_tree(left); // Pass the cloned left to invert_tree
    }

    root
}
