use multihash::{Code, MultihashDigest};
use std::time::SystemTime;

// Here I define the structure of each type of node that can be stored in the file system tree
pub enum Metadata {
    Directory(DirectoryMetadata),
    DataBlock(DataBlock),
}

pub struct FileMetadata {
    cid: Vec<u8>,
    size: u64,
    file_name: Option<String>,
    content_type: Option<String>,
    created: SystemTime,
    modified: SystemTime,
    accessed: SystemTime,
}

impl FileMetadata {
    pub fn new(data: &[u8], file_name: Option<String>, content_type: Option<String>) -> Self {
        let cid = Code::Sha2_256.digest(data).to_bytes();
        let now = SystemTime::now();
        FileMetadata {
            cid,
            size: data.len() as u64,
            file_name,
            content_type,
            created: now,
            modified: now,
            accessed: now,
        }
    }
}

pub struct DirectoryMetadata {
    cid: Vec<u8>,

    created: SystemTime,
    modified: SystemTime,
    accessed: SystemTime,
}

impl DirectoryMetadata {
    pub fn new(data: &[u8]) -> Self {
        let cid = Code::Sha2_256.digest(data).to_bytes();
        let now = SystemTime::now();
        DirectoryMetadata {
            cid,
            entries: Vec::new(),
            created: now,
            modified: now,
            accessed: now,
        }
    }
}

pub struct DataBlock {
    cid: Vec<u8>,
    data: Vec<u8>,
}

impl DataBlock {
    pub fn new(data: &[u8]) -> Self {
        let cid = Code::Sha2_256.digest(data).to_bytes();
        DataBlock {
            cid,
            data: data.to_vec(),
        }
    }
}


// Implementing methods for the Tree node structure that are specific to the file system
impl PartialEq for TreeNode<Metadata> {
    fn eq(&self, other: &Self) -> bool {
        self.data.cid == other.data.cid
    }

    /// Adds a child to the current node.
    ///
    /// # Parameters
    ///
    /// * `parent` - The existing node in the tree that we are adding the child to.
    /// 
    /// * `child` - The new node that we are adding to the tree.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - Returns an error if request did not follow node hierachy.

}

/// Adds a child to the current node.
///
/// # Parameters
///
/// * `parent` - The existing node in the tree that we are adding the child to.
/// 
/// * `child` - The new node that we are adding to the tree.
///
/// # Returns
///
/// * `Result<(), String>` - Returns Ok if successful, string if error.
/// 
/// # Errors
/// 
/// * If Node Hierarchy is not followed, returns an error
pub fn add_child(parent: &mut TreeNode<Metadata>, child: TreeNode<Metadata>) -> Result<(), String> {
    match &mut parent.data {
        Metadata::Directory(parent_directory) => {
            match &child.data {
                Metadata::Directory(_) | Metadata::DataBlock(_) => {
                    return Err("DataBlock cannot be added to a DirectoryMetadata node.".to_string());
                }
                Metadata::File(child_file) => {
                    match child.children.iter().all(|child| matches!(child.data, Metadata::DataBlock(_))) {
                        true => parent.children.push(child),
                        false => return Err("FileMetadata nodes can only have DataBlock nodes as children.".to_string()),
                    }
                }
            }
        }
        Metadata::File(parent_file) => {
            match child.data {
                Metadata::DataBlock(_) => parent.children.push(child),
                _ => return Err("FileMetadata nodes can only have DataBlock nodes as children.".to_string()),
            }
        }
        Metadata::DataBlock(_) => {
            return Err("DataBlock nodes cannot have children.".to_string());
        }
    }

    Ok(())
}



