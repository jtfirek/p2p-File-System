use crate::tree::TreeNode;
use std::cmp::PartialEq;

pub fn dfs<T: PartialEq>(root: &TreeNode<T>, target: &TreeNode<T> ) -> bool where TreeNode<T>: PartialEq {
    if *root == *target {
        return true;
    }
    for child in &root.children {
        if dfs( child, target) {
            return true;
        }
    }
    false
}