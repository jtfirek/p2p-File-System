use crate::tree::TreeNode;

pub fn dfs<T: std::cmp::PartialEq>(root: &TreeNode<T>, target: &TreeNode<T> ) -> bool {
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