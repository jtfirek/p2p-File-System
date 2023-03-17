# IPFSish project 

Implementing a distributed file system like IPFS using rust and libp2p to learn more about how to actually build peer to peer networks.

If you are looking through project and see something you would do differently or things I could to keep building on this project please make a github issue so I can keep learning!

## Project Structure

The project is organized into several crates, each focusing on a specific aspect of the file system:

- `dfs_core`: The core functionality of the file system, including the primary data structures and algorithms for the storage layer.
- `dfs_storage`: Handles the actual storage of data, such as writing to and reading from the disk. This crate manages data persistence, caching, and other low-level storage operations.
- `dfs_network`: Manages the networking aspect of the file system, using libp2p for peer discovery, connection establishment, and data exchange between peers.
- `dfs_api`: Exposes a high-level API for interacting with the file system, allowing users or other applications to perform operations like creating files, reading files, and listing directories.
- `dfs_cli`: Provides a command-line interface for users to interact with the file system, using the `dfs_api` crate to perform operations and display results.
- `dfs_thread_pool`: A utility crate that provides a thread pool implementation to efficiently manage concurrent tasks in other parts of the system.

## Getting Started

To build the project, ensure you have Rust and Cargo installed on your system. Then, navigate to the root directory of the workspace and run the following command:

```sh
cargo build
