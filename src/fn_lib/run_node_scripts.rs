use super::type_node_pac::NodePackageMannegerType;
pub struct ReturnCoomad {
    pub script: String,
    pub args: String,
}
pub fn run_node_scripts(package_type: NodePackageMannegerType, scripts: String) -> ReturnCoomad {
    let package_script = match package_type {
        NodePackageMannegerType::Npm => "npm",
        NodePackageMannegerType::Unknown => "npm",
        NodePackageMannegerType::Yarn => "yarn",
        NodePackageMannegerType::Pnpm => "pnpm",
    };

    let command = format!(
        "{} {} {}",
        package_script.to_string(),
        &"run".to_string(),
        &scripts.to_string()
    );

    return ReturnCoomad {
        script: package_script.to_string(),
        args: scripts.to_string(),
    };
}
