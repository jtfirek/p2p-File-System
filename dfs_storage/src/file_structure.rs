use bincode;
use multihash::{Code, MultihashDigest};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub type CID = Vec<u8>;

// Here I define the structure of each type of block that can be stored in the file system tree

pub enum Metadata {
    Directory(Directory),
    File(File),
    DataBlock(DataBlock),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Directory {
    pub cid: CID,
    pub directory_name: Option<String>,
    pub created: SystemTime,
    pub size: u64, // represent the number of files in the directory
    pub entries: Vec<CID>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct File {
    pub cid: CID,
    pub file_name: Option<String>,
    pub created: SystemTime,
    pub size: u64, // represent the number of datablocks in the file
    pub data_blocks: Vec<CID>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataBlock {
    pub cid: CID,
    pub data: Vec<u8>, // binary data stored in an array of bytes 
}

// contructor methods that also calculate the CID of the block for each type of block
impl Directory {
    pub fn new_directory(directory_name: Option<String>) -> Self {
        let now = SystemTime::now();
        let mut directory_metadata = Directory {
            cid: vec![],
            directory_name,
            created: now,
            size: 0,
            entries: Vec::<CID>::new(),
        };
        let encoded_bytes = bincode::serialize(&directory_metadata).unwrap();
        let cid = Code::Sha2_256.digest(&encoded_bytes).to_bytes();
        directory_metadata.cid = cid;
        directory_metadata
    }
}

impl File {
    pub fn new_file( file_name: Option<String>, data_blocks: Vec<CID>) -> Self {
        let now = SystemTime::now();
        let mut file_metadata = File {
            cid: vec![],
            file_name,
            created: now,
            size: data_blocks.len() as u64,
            data_blocks,
        };
        let encoded_bytes = bincode::serialize(&file_metadata).unwrap();
        let cid = Code::Sha2_256.digest(&encoded_bytes).to_bytes();
        file_metadata.cid = cid;
        file_metadata
    }
}

impl DataBlock {
    pub fn new(data: &[u8]) -> Self {
        // unlike others, hash the data directly
        let cid = Code::Sha2_256.digest(data).to_bytes();
        DataBlock {
            cid,
            data: data.to_vec(),
        }
    }
}





