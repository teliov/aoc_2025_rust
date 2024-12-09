pub fn read_file(filename: &String) -> String {
    std::fs::read_to_string(filename).expect("Failed to read file")
}