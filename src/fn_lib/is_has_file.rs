use std::fs;

pub fn is_file_exists_in_directory(directory_path: &str, file_name: &str) -> bool {
    let dir = fs::read_dir(directory_path).unwrap();
    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && path.file_name().unwrap() == file_name {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_exists_in_directory() {
        let directory_path = "test_files/";
        let file_name = "package.json";
        let is_file_exists = is_file_exists_in_directory(directory_path, file_name);
        assert_eq!(is_file_exists, true);
    }
    #[test]
    fn test_is_file_exists_in_directory_false() {
        let directory_path = "./";
        let file_name = "package.json2";
        let is_file_exists = is_file_exists_in_directory(directory_path, file_name);
        assert_eq!(is_file_exists, false);
    }
    #[test]
    fn test_is_file_exists_in_directory_false_2() {
        let directory_path = "test_files/";
        let file_name = "not_fond_file.json2";
        let is_file_exists = is_file_exists_in_directory(directory_path, file_name);
        assert_eq!(is_file_exists, false);
    }
}
