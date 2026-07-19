use std::path::PathBuf;

#[derive(Debug)]
pub struct EditRequest {
    pub instruction: String,
    pub selection: String,
    pub context: String,
    pub file_path: PathBuf,
    pub file_type: String,
}
