use std::path::Path;

use super::{package_commands::NodePackageMannegerType, type_node_pac::detect_package_manager};

pub fn check_installde_package_maneger(
    package_manneger: &Option<NodePackageMannegerType>,
    folder_path: Option<&Path>,
) -> NodePackageMannegerType {
    let package_manager = match package_manneger {
        None => detect_package_manager(&folder_path.expect("./").to_str().unwrap()),
        Some(v) => v.clone(),
    };
    match package_manager.is_installed_command() {
        true => package_manager,
        false => {
            println!(
                "{} is not installed",
                package_manager.get_commands().command_name
            );
            std::process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_check_installed_package_manager() {
        let folder_path = Some(PathBuf::from("/path/to/folder"));
        let package_manager = NodePackageMannegerType::Npm; // Set the package manager to Npm for this test
        let result =
            check_installde_package_maneger(&Some(package_manager.clone()), folder_path.as_deref());

        // Check that the result matches the expected package manager
        assert_eq!(result, package_manager);
    }
}
