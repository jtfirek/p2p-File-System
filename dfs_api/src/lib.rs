use dfs_storage::CID;




pub struct DFSApi {
    // The file_system field should be replaced with your actual file_system implementation
    file_system: (),
}

impl DFSApi {
    pub  fn new() -> Result<Self> {
        let file_system = ();
        // Initialize your file_system here

        Ok(Self { file_system })
    }

    // directory can either be created at root with no parent or in an exisiting directory 
    pub fn create_directory(&mut self, parent: Option<CID>) -> Result<()> {
        
    }

    // Unlike create_directory, create_file can only be created in an existing directory
    pub fn create_file(&mut self, directory: CID) -> Result<()> {
        // Implement the create_file functionality here
        Ok(())
    }

    // given a files CID, read the file and return the data
    pub fn read_file(&self, file: CID) -> Result<Vec<u8>> {
        // Implement the read_file functionality here
        Ok(Vec::new())
    }

    // given a directory CID, list the files and directories in the directory
    pub fn list_directory(&self, directory: CID) -> Result<Vec<String>> {
        // Implement the list_directory functionality here
        Ok(Vec::new())
    }

    // given a file CID, write the data to the file
    pub fn write_file(&mut self, data: &[u8]) -> Result<()> {
        // Implement the write_file functionality here
        Ok(())
    }

    // given a files parent directory CID, delete the file
    pub fn delete_file(&mut self, parent: CID) -> Result<()> {
        // Implement the delete_file functionality here
        Ok(())
    }

    // given a directory parent CID delete the directory
    pub fn delete_directory(&mut self, parent: CID) -> Result<()> {
        // Implement the delete_directory functionality here
        Ok(())
    }

}
