use std::path::{Path, PathBuf};

pub fn get_directory_from_file_path(file_path: &PathBuf) -> Option<&Path> {
    let path = file_path;

    if path.is_dir() {
        Some(path)
    } else {
        let path = Path::new(file_path);
        path.parent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_directory_from_file_path() {
        let file_path: PathBuf = ["./package.json"].iter().collect();
        let directory = get_directory_from_file_path(&file_path);
        assert_eq!(directory, Some(Path::new(".")));
    }
    #[test]
    fn test_get_directory_from_file_path_long() {
        let file_path: PathBuf = ["aaa/bbb/ccc/ddd/package.json"].iter().collect();
        let directory = get_directory_from_file_path(&file_path);
        assert_eq!(directory, Some(Path::new("aaa/bbb/ccc/ddd")));
    }
    #[test]
    fn test_get_directory_from_file_path_long_2() {
        let file_path: PathBuf = ["./aaa/bbb/ccc/"].iter().collect();
        let directory = get_directory_from_file_path(&file_path);
        assert_eq!(directory, Some(Path::new("./aaa/bbb")));
    }
    #[test]
    fn test_get_directory_from_file_path_long_3() {
        let file_path: PathBuf = ["aaa/bbb/ccc/"].iter().collect();
        let directory = get_directory_from_file_path(&file_path);
        assert_eq!(directory, Some(Path::new("aaa/bbb")));
    }
}
