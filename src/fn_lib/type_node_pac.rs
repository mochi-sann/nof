use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, clap::ValueEnum, Clone)]
pub enum NodePackageMannegerType {
    Npm,
    Yarn,
    Pnpm,
    Unknown,
}

pub fn detect_package_manager(project_dir: &str) -> NodePackageMannegerType {
    let package_file_path = Path::new(project_dir).join("package.json");

    let mut package_file = match File::open(&package_file_path) {
        Ok(file) => file,
        Err(_) => return NodePackageMannegerType::Unknown,
    };

    let mut package_file_contents = String::new();
    if let Err(_) = package_file.read_to_string(&mut package_file_contents) {
        return NodePackageMannegerType::Unknown;
    }

    let package_json: serde_json::Value = match serde_json::from_str(&package_file_contents) {
        Ok(json) => json,
        Err(_) => return NodePackageMannegerType::Unknown,
    };

    if Path::new(project_dir).join("yarn.lock").exists() {
        NodePackageMannegerType::Yarn
    } else if Path::new(project_dir).join("pnpm-lock.yaml").exists() {
        NodePackageMannegerType::Pnpm
    } else if Path::new(project_dir).join("package-lock.json").exists() {
        NodePackageMannegerType::Npm
    } else if package_json["dependencies"].is_object() {
        NodePackageMannegerType::Npm
    } else if package_json["devDependencies"].is_object() {
        NodePackageMannegerType::Yarn
    } else {
        NodePackageMannegerType::Npm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_package_manager_npm() {
        let project_dir = "./test_files/package_test/npm";
        let package_manager = detect_package_manager(project_dir);
        assert_eq!(package_manager, NodePackageMannegerType::Npm);
    }
    #[test]
    fn test_detect_package_manager_yarn() {
        let project_dir = "./test_files/package_test/yarn";
        let package_manager = detect_package_manager(project_dir);
        assert_eq!(package_manager, NodePackageMannegerType::Yarn);
    }
    #[test]
    fn test_detect_package_manager_pnpm() {
        let project_dir = "./test_files/package_test/pnpm";
        let package_manager = detect_package_manager(project_dir);
        assert_eq!(package_manager, NodePackageMannegerType::Pnpm);
    }
}
