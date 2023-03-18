# Centralized Server Project

The Centralized Server Project is a Rust-based project that I am doing a a first step to building a Peer to Peer Distributed File System.

If you have any suggestions or feedback on how to improve the project or things you would do differently, please create a GitHub issue to share your thoughts.

## Project Structure

The project is organized into several crates, each focusing on a specific aspect of the file server:

- `server_core`: The core functionality of the file server, including the primary data structures and algorithms for the storage layer.
- `server_storage`: Handles the actual storage of data, such as writing to and reading from the disk. This crate manages data persistence, caching, and other low-level storage operations.
- `server_network`: Manages the networking aspect of the file server, using Rust's standard networking libraries for client-server communication and data exchange.
- `server_api`: Exposes a high-level API for interacting with the file server, allowing users or other applications to perform operations like creating files, reading files, and listing directories.
- `server_cli`: Provides a command-line interface for users to interact with the file server, using the `server_api` crate to perform operations and display results.
- `server_thread_pool`: A utility crate that provides a thread pool implementation to efficiently manage concurrent tasks in other parts of the system.

## Getting Started

To build the project, ensure you have Rust and Cargo installed on your system. Then, navigate to the root directory of the workspace and run the following command:

```sh
cargo build
