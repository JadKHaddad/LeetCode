use std::{cell::RefCell, cmp::max, rc::Rc};

fn main() {}

// Definition for a binary tree node.
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

    fn max_depth(&self) -> i32 {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                1 + max(left.borrow().max_depth(), right.borrow().max_depth())
            }
            (Some(left), None) => 1 + left.borrow().max_depth(),
            (None, Some(right)) => 1 + right.borrow().max_depth(),
            (None, None) => 1,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |node| node.borrow().max_depth())
    }
}
