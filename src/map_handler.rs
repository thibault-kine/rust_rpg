use std::fs;

pub fn load_map(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("Failed to read map file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}