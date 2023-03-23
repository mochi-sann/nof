#[derive(Debug, PartialEq, clap::ValueEnum, Clone)]
pub enum NodePackageMannegerType {
    Npm,
    Yarn,
    Pnpm,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnCoomad {
    pub script: String,
    pub args: String,
}

impl NodePackageMannegerType {
    pub fn run_node_scripts(&self, scripts: String) -> ReturnCoomad {
        let package_script = match self {
            NodePackageMannegerType::Npm => "npm run",
            NodePackageMannegerType::Yarn => "yarn run",
            NodePackageMannegerType::Pnpm => "pnpm run",
        };

        return ReturnCoomad {
            script: package_script.to_string(),
            args: scripts.to_string(),
        };
    }
    pub fn install_command(&self, lib: String) -> ReturnCoomad {
        let package_script = match self {
            NodePackageMannegerType::Npm => "npm install",
            NodePackageMannegerType::Yarn => "yarn install",
            NodePackageMannegerType::Pnpm => "pnpm install",
        };
        return ReturnCoomad {
            script: package_script.to_string(),
            args: lib.to_string(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_package_manager_run_command_npm() {
        let package_manager = NodePackageMannegerType::Npm;
        assert_eq!(
            package_manager.run_node_scripts("test".to_string()),
            ReturnCoomad {
                script: "npm run".to_string(),
                args: "test".to_string(),
            }
        );
    }
    #[test]
    fn test_package_manager_run_command_yarn() {
        let package_manager = NodePackageMannegerType::Yarn;
        assert_eq!(
            package_manager.run_node_scripts("test".to_string()),
            ReturnCoomad {
                script: "yarn run".to_string(),
                args: "test".to_string(),
            }
        );
    }
    #[test]
    fn test_package_manager_run_command_pnpm() {
        let package_manager = NodePackageMannegerType::Pnpm;
        assert_eq!(
            package_manager.run_node_scripts("test".to_string()),
            ReturnCoomad {
                script: "pnpm run".to_string(),
                args: "test".to_string(),
            }
        );
    }

    #[test]
    fn test_package_manager_install_command_npm() {
        let package_manager = NodePackageMannegerType::Npm;
        assert_eq!(
            package_manager.install_command("test".to_string()),
            ReturnCoomad {
                script: "npm install".to_string(),
                args: "test".to_string(),
            }
        );
    }
    #[test]
    fn test_package_manager_install_command_yarn() {
        let package_manager = NodePackageMannegerType::Yarn;
        assert_eq!(
            package_manager.install_command("test".to_string()),
            ReturnCoomad {
                script: "yarn install".to_string(),
                args: "test".to_string(),
            }
        );
    }
    #[test]
    fn test_package_manager_install_command_pnpm() {
        let package_manager = NodePackageMannegerType::Pnpm;
        assert_eq!(
            package_manager.install_command("test".to_string()),
            ReturnCoomad {
                script: "pnpm install".to_string(),
                args: "test".to_string(),
            }
        );
    }
    #[test]
    fn test_package_manager_install_command_pnpm_none_lib() {
        let package_manager = NodePackageMannegerType::Pnpm;
        assert_eq!(
            package_manager.install_command("".to_string()),
            ReturnCoomad {
                script: "pnpm install".to_string(),
                args: "".to_string(),
            }
        );
    }
}
