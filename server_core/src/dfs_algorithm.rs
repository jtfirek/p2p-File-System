use crate::tree::TreeNode;
use std::cmp::PartialEq;
use std::sync::{Arc, Mutex};

pub fn dfs<T: PartialEq>(root: &TreeNode<T>, target: &TreeNode<T> ) -> bool where TreeNode<T>: PartialEq {
    let root_locked = Arc::new(Mutex::new(root));
    if *root == *target {
        return true;
    }
    for child in &root.children {
        let child_locked = Arc::new(Mutex::new(child));
        if dfs( child, target) {
            return true;
        }
    }
    false
}