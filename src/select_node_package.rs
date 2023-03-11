use std::fs;

fn is_file_exists(dir_path: &str, file_name: &str) -> bool {
    let dir = fs::read_dir(dir_path).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == file_name {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_exists() {
        let dir_path = "./test_files";
        let file_name = "aaa.json";
        let expected = true;
        let actual = is_file_exists(dir_path, file_name);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_is_file_exists_2() {
        let dir_path = "./test_files";
        let file_name = "not_fond_files.json";
        let expected = false;
        let actual = is_file_exists(dir_path, file_name);
        assert_eq!(expected, actual);
    }
}


