# IPFSish project 

Implementing a distributed file system like IPFS using rust and libp2p to learn more about how to actually build peer to peer networks.

If you are looking through project and see something you would do differently or things I could to keep building on this project please make a github issue so I can keep learning!

# Project Structure

The project is organized into several crates, each focusing on a specific aspect of the file system:

## dfs_core
The core primary data structures and algorithms for the storage layer are stored in the `dfs_core` crate. A genetric tree and dfs implementation are provided here. 

## dfs_storage
the `dfs_storage` crate handles the actual storage of data, such as writing to and reading from the disk. This crate manages data persistence, caching, and other low-level storage operations.

The three structs are `FileMetadata`, `DirectoryMetadata`, and `DataBlock`. All structs have a `cid` generated from the data block's content using the SHA2-256 hash function to be used in content based addressing.

### Node Hierarchy and Relationships
- `DirectoryMetadata` nodes represent directories in the file system. They can have both `FileMetadata` and `DirectoryMetadata` structs as children.
- `FileMetadata` nodes represent files in the file system. They can only have `DataBlock` structs as children. Each `DataBlock` child represents a portion of the file's content. The file's content can be reconstructed by concatenating the data from all of its associated `DataBlock` nodes in the correct order.
- `DataBlock` nodes represent individual data blocks in the file system. They cannot have any children. `DataBlock`vnodes are the lowest level in the hierarchy and only store actual data bytes.


- `dfs_network`: Manages the networking aspect of the file system, using libp2p for peer discovery, connection establishment, and data exchange between peers.
- `dfs_api`: Exposes a high-level API for interacting with the file system, allowing users or other applications to perform operations like creating files, reading files, and listing directories.
- `dfs_cli`: Provides a command-line interface for users to interact with the file system, using the `dfs_api` crate to perform operations and display results.
- `dfs_thread_pool`: A utility crate that provides a thread pool implementation to efficiently manage concurrent tasks in other parts of the system.

## Getting Started

To build the project, ensure you have Rust and Cargo installed on your system. Then, navigate to the root directory of the workspace and run the following command:

```sh
cargo build