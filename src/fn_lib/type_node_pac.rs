use std::path::Path;

use super::package_commands::NodePackageMannegerType;

pub fn detect_package_manager(project_dir: &str) -> NodePackageMannegerType {
    let _package_file_path = Path::new(project_dir).join("package.json");

    if Path::new(project_dir).join("yarn.lock").exists() {
        NodePackageMannegerType::Yarn
    } else if Path::new(project_dir).join("pnpm-lock.yaml").exists() {
        NodePackageMannegerType::Pnpm
    } else if Path::new(project_dir).join("package-lock.json").exists() {
        NodePackageMannegerType::Npm
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
