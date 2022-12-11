use std::fs;

pub fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Should have been able to read file")
}
