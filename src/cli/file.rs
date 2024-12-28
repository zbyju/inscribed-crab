use std::fs;

pub fn read_file(file_path: &str) -> Result<String, String> {
    let content = fs::read_to_string(file_path);

    content.map_err(|e| e.to_string())
}
