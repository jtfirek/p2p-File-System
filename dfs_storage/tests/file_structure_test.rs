
use dfs_storage::{
    Directory, File, DataBlock, Metadata,
};
use bincode;
use multihash::{Code, MultihashDigest};

// Testing the creation of each type of block
#[test]
fn test_directory_creation() {
    let directory_name = Some("test_directory".to_string());
    let directory = Directory::new_directory(directory_name.clone());
    assert_eq!(directory.directory_name, directory_name);
    assert_eq!(directory.size, 0);
    assert!(directory.entries.is_empty());
    // test if the CID was actually calculated
    assert!(directory.cid.len() > 0);
}

#[test]
fn test_file_creation() {
    let file_name = Some("test_file.txt".to_string());
    let data_blocks = Vec::new();
    let file = File::new_file(file_name.clone(), data_blocks.clone());
    assert_eq!(file.file_name, file_name);
    assert_eq!(file.size, 0);
    assert_eq!(file.data_blocks, data_blocks);
}

#[test]
fn test_data_block_creation() {
    let data = b"Hello, world!";
    let data_block = DataBlock::new(data);
    assert_eq!(data_block.data, data.to_vec());
}

// Testing the serialization and deserialization of each type of block and the PartialEq trait
#[test]
fn test_directory_serialization_deserialization() {
    let directory_name = Some("test_directory".to_string());
    let directory = Directory::new_directory(directory_name.clone());
    let encoded = bincode::serialize(&directory).unwrap();
    let decoded: Directory = bincode::deserialize(&encoded).unwrap();
    assert_eq!(directory, decoded);
}
#[test]
fn test_file_serialization_deserialization() {
    let file_name = Some("test_file.txt".to_string());
    let data_blocks = Vec::new();
    let file = File::new_file(file_name.clone(), data_blocks.clone());
    let encoded = bincode::serialize(&file).unwrap();
    let decoded: File = bincode::deserialize(&encoded).unwrap();
    assert_eq!(file, decoded);
}
#[test]
fn test_data_block_cid() {
    let data = b"Hello, world!";
    let data_block = DataBlock::new(data);
    let cid = Code::Sha2_256.digest(data).to_bytes();
    assert_eq!(data_block.cid, cid);
}

#[test]
fn test_metadata_enum() {
    let directory_name = Some("test_directory".to_string());
    let directory = Directory::new_directory(directory_name.clone());
    let file_name = Some("test_file.txt".to_string());
    let data_blocks = Vec::new();
    let file = File::new_file(file_name.clone(), data_blocks.clone());
    let data = b"Hello, world!";
    let data_block = DataBlock::new(data);

    let metadata_directory = Metadata::Directory(directory);
    let metadata_file = Metadata::File(file);
    let metadata_data_block = Metadata::DataBlock(data_block);

    match metadata_directory {
        Metadata::Directory(dir) => assert_eq!(dir.directory_name, directory_name),
        _ => panic!("Expected a Metadata::Directory variant"),
    }

    match metadata_file {
        Metadata::File(f) => {
            assert_eq!(f.file_name, file_name);
            assert_eq!(f.data_blocks, data_blocks);
        }
        _ => panic!("Expected a Metadata::File variant"),
    }

    match metadata_data_block {
        Metadata::DataBlock(db) => assert_eq!(db.data, data.to_vec()),
        _ => panic!("Expected a Metadata::DataBlock variant"),
    }
}