use std::path::{Path, PathBuf};

use super::{get_directory_from_file_path, is_has_file};

pub enum NodePackageMannegerType {
    Yarn,
    Npm,
    Pnpm,
}

struct NodePackageManneger {
    file_path: String,
}

impl NodePackageManneger {
    fn get_node_package_Manneger(&self) -> NodePackageMannegerType {
        let folder_path =
            get_directory_from_file_path::get_directory_from_file_path(&self.file_path);

        if is_has_file::is_file_exists_in_directory(path_to_str(folder_path), "yarn.lock") {
            return NodePackageMannegerType::Yarn;
        } else if is_has_file::is_file_exists_in_directory(
            path_to_str(folder_path),
            "package-lock.json",
        ) {
            return NodePackageMannegerType::Npm;
        } else if is_has_file::is_file_exists_in_directory(
            path_to_str(folder_path),
            "pnpm-lock.yaml",
        ) {
            return NodePackageMannegerType::Pnpm;
        } else {
            return NodePackageMannegerType::Yarn;
        }
    }
}

fn path_to_str<'a>(path: Option<&'a Path>) -> &'a str {
    match path {
        Some(p) => match PathBuf::from(p).to_str() {
            Some(s) => s,
            None => "",
        },
        None => "",
    }
}
