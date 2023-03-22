use bincode;
use multihash::{Code, MultihashDigest};
use std::time::SystemTime;

pub type CID = Vec<u8>;

// Here I define the structure of each type of block that can be stored in the file system tree
pub enum Metadata {
    Directory {
        cid: CID,
        directory_name: Option<String>,
        created: SystemTime,
        size: u64, // represent the number of files in the directory
        entries: Vec<CID>,
    },
    File {
        cid: CID,
        file_name: Option<String>,
        created: SystemTime,
        size: u64, // represent the number of datablocks in the file
        data_blocks: Vec<CID>,
    },
    DataBlock {
        cid: CID,
        data: Vec<u8>, // binary data stored in an array of bytes 
    },
}

// contructor methods that also calculate the CID of the block for each type of block
impl Metadata {
    pub fn new_directory(directory_name: Option<String>) -> Self {
        let now = SystemTime::now();
        let mut directory_metadata = Metadata::Directory {
            cid: vec![],
            directory_name,
            created: now,
            size: 0,
            entries: Vec::<CID>::new(),
        };
        let encoded = bincode::serialize(&directory_metadata).unwrap();
        let cid = Code::Sha2_256.digest(&encoded).into_bytes();
        directory_metadata.cid = cid;
        directory_metadata
    }

    pub fn new_file( file_name: Option<String>, data_blocks: Vec<CID>) -> Self {
        let now = SystemTime::now();
        let mut file_meta = Metadata::File {
            cid: vec![],
            file_name,
            created: now,
            size: data_blocks.len() as u64,
            data_blocks,
        };
        let encoded = bincode::serialize(&file_meta).unwrap();
        let cid = Code::Sha2_256.digest(&encoded).into_bytes();
        file_metadata.cid = cid;
        file_metadata
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
    
}





