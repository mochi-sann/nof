use std::path::{Component, Path};

pub fn get_directory_from_file_path(file_path: &str) -> Option<&Path> {
    let path = Path::new(file_path);

    if path.is_dir() {
        return Some(path);
    } else {
        let path = Path::new(file_path);
        return path.parent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_directory_from_file_path() {
        let file_path = "./package.json";
        let directory = get_directory_from_file_path(file_path);
        assert_eq!(directory, Some(Path::new(".")));
    }
    #[test]
    fn test_get_directory_from_file_path_long() {
        let file_path = "aaa/bbb/ccc/ddd/package.json";
        let directory = get_directory_from_file_path(file_path);
        assert_eq!(directory, Some(Path::new("aaa/bbb/ccc/ddd")));
    }
    #[test]
    fn test_get_directory_from_file_path_long_2() {
        let file_path = "./aaa/bbb/ccc/";
        let directory = get_directory_from_file_path(file_path);
        assert_eq!(directory, Some(Path::new("aaa/bbb")));
    }
}
