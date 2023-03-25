
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::time::{Duration, SystemTime};

//     #[test]
//     fn test_directory_creation() {
//         let directory_name = Some(String::from("test_directory"));
//         let directory = Metadata::new_directory(directory_name.clone());

//         if let Metadata::Directory {
//             cid,
//             directory_name: dir_name,
//             created,
//             size,
//             entries,
//         } = directory
//         {
//             assert!(dir_name.is_some());
//             assert_eq!(dir_name.unwrap(), "test_directory");
//             assert_eq!(size, 0);
//             assert!(entries.is_empty());

//             let now = SystemTime::now();
//             let duration = now.duration_since(created).unwrap();
//             assert!(duration < Duration::from_secs(1));
//         } else {
//             panic!("Expected Metadata::Directory");
//         }
//     }

//     #[test]
//     fn test_file_creation() {
//         let file_name = Some(String::from("test_file"));
//         let data_blocks = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
//         let cids: Vec<CID> = data_blocks
//             .iter()
//             .map(|data| Code::Sha2_256.digest(data).to_bytes())
//             .collect();

//         let file = Metadata::new_file(file_name.clone(), cids.clone());

//         if let Metadata::File {
//             cid,
//             file_name: f_name,
//             created,
//             size,
//             data_blocks: file_data_blocks,
//         } = file
//         {
//             assert!(f_name.is_some());
//             assert_eq!(f_name.unwrap(), "test_file");
//             assert_eq!(size, data_blocks.len() as u64);
//             assert_eq!(file_data_blocks, cids);

//             let now = SystemTime::now();
//             let duration = now.duration_since(created).unwrap();
//             assert!(duration < Duration::from_secs(1));
//         } else {
//             panic!("Expected Metadata::File");
//         }
//     }

//     #[test]
//     fn test_data_block_creation() {
//         let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
//         let data_block = DataBlock::new(&data);

//         assert_eq!(data_block.cid, Code::Sha2_256.digest(&data).to_bytes());
//         assert_eq!(data_block.data, data);
//     }
// }