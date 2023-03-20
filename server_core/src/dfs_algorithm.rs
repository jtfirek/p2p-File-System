use crate::tree::TreeNode;
use std::cmp::PartialEq;

pub fn dfs<T: PartialEq>(root: &TreeNode<T>, target: &TreeNode<T> ) -> Option<&TreeNode<T>> where TreeNode<T>: PartialEq {
    if *root == *target {
        return root;
    }
    for child in &root.children {
        if dfs( child, target) {
            return dfs;
        }
    }
    false
}