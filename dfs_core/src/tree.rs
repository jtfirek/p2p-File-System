use std::sync::atomic::{AtomicUsize, Ordering};

/// A generic tree node that holds a value of type `T`, a unique identifier, and a list of its children.
pub struct TreeNode<T> {
    id: usize,
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

/// A global atomic counter used to generate unique identifiers for tree nodes.
static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

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
        let id = NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        TreeNode {
            id,
            value,
            children: Vec::new(),
        }
    }
}

/// Implements the `PartialEq` trait for `TreeNode` using the unique identifier.
impl<T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
