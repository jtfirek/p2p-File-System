use multihash::{Code, MultihashDigest};
use std::time::SystemTime;

pub enum Metadata {
    File(FileMetadata),
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

impl PartialEq for TreeNode<Metadata> {
    fn eq(&self, other: &Self) -> bool {
        self.data.cid == other.data.cid
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