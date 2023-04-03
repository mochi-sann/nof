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
                "Package {} manager not installed",
                package_manager.get_commands().command_name
            );
            std::process::exit(1);
        }
    }
}
