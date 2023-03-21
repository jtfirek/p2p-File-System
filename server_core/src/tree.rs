
/// A generic tree node that holds data of type `T` and a list of its children.
#[derive(Debug)]
pub struct TreeNode<T> {
    pub data: T,
    pub children: Vec<TreeNode<T>>,
}


impl<T> TreeNode<T> {
    /// Creates a new `TreeNode` with the given value and no children.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to be stored in the new tree node.
    ///
    /// # Returns
    ///
    /// A new `TreeNode` instance with the given data and no children.
    pub fn new(data: T) -> Self {
        TreeNode {
            data,
            children: Vec::new(),
        }
    }
}

impl<T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Clone> Clone for TreeNode<T> {
    fn clone(&self) -> Self {
        TreeNode {
            data: self.data.clone(),
            children: self.children.clone(),
        }
    }
}