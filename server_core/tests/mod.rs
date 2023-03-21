#[cfg(test)]
mod tests {
    use server_core::tree::TreeNode;
    use server_core::dfs_algorithm::dfs;
    #[test]
    fn test_tree_node_creation() {
        let data = 42;
        let node = TreeNode::new(data);

        assert_eq!(node.data, data);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_dfs() {
        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);

        let mut root = TreeNode::new(0);
        root.children.push(node1);
        root.children.push(node2);
        root.children[0].children.push(node3);
        root.children[1].children.push(node4);

        let target1 = TreeNode::new(3);
        let target2 = TreeNode::new(5);

        assert_eq!(dfs(&root, &target1).unwrap().data, 3);
        assert_eq!(dfs(&root, &target2), None);
    }
}