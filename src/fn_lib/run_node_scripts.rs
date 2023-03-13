use std::process::Command;

use super::type_node_pac::NodePackageMannegerType;

pub fn run_node_scripts(package_type: NodePackageMannegerType, scripts: String) {
    let package_script = match package_type {
        NodePackageMannegerType::Npm => "npm",
        NodePackageMannegerType::Unknown => "npm",
        NodePackageMannegerType::Yarn => "yarn",
        NodePackageMannegerType::Pnpm => "pnpm",
    };
    println!("package_script: {:?}", package_script);

    let command_output = Command::new(package_script.to_string())
        .arg("run")
        .arg(scripts)
        .spawn()
        .expect("Failed to run node scripts");

    println!("command_output: {:?}", command_output.stdout)
}
