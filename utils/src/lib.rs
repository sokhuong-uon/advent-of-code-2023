use std::fs;
pub fn get_file_content(file_path: &str) -> String {
    let file_path = if file_path.starts_with("/") {
        file_path.to_string()
    } else {
        format!("/{}", file_path)
    };
    let dir = std::env::current_dir().unwrap();
    fs::read_to_string(format!("{}{}", dir.display(), file_path)).unwrap()
}
