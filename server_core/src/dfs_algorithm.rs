use crate::tree::TreeNode;
use std::cmp::PartialEq;

pub fn dfs<T: PartialEq + Clone>(root: &TreeNode<T>, target: &TreeNode<T> ) -> Option<TreeNode<T>> where TreeNode<T>: PartialEq {
    if *root == *target {
        return Some((*target).clone());
    }
    for child in &root.children {
        let result = dfs(child, target);
        if result.is_some() {
            return result;
        }
    }
    None
}