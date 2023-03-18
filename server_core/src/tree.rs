
/// A generic tree node that holds a value of type `T` and a list of its children.
pub struct TreeNode<T> {
    pub value: T,
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
    /// A new `TreeNode` instance with the given value, a unique identifier, and no children.
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }
}
