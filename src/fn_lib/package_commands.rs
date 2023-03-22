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
    pub fn install_command(&self) -> &str {
        match self {
            NodePackageMannegerType::Npm => "npm install",
            NodePackageMannegerType::Yarn => "yarn install",
            NodePackageMannegerType::Pnpm => "pnpm install",
        }
    }

    pub fn update_command(&self) -> &str {
        match self {
            NodePackageMannegerType::Npm => "npm update",
            NodePackageMannegerType::Yarn => "yarn upgrade",
            NodePackageMannegerType::Pnpm => "pnpm update",
        }
    }

    fn uninstall_command(&self) -> &str {
        match self {
            NodePackageMannegerType::Npm => "npm uninstall",
            NodePackageMannegerType::Yarn => "yarn remove",
            NodePackageMannegerType::Pnpm => "pnpm remove",
        }
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
    fn test_package_manager_install_command() {
        assert_eq!(
            NodePackageMannegerType::Npm.install_command(),
            "npm install"
        );
        assert_eq!(
            NodePackageMannegerType::Yarn.install_command(),
            "yarn install"
        );
        assert_eq!(
            NodePackageMannegerType::Pnpm.install_command(),
            "pnpm install"
        );
    }

    #[test]
    fn test_package_manager_update_command() {
        assert_eq!(NodePackageMannegerType::Npm.update_command(), "npm update");
        assert_eq!(
            NodePackageMannegerType::Yarn.update_command(),
            "yarn upgrade"
        );
        assert_eq!(
            NodePackageMannegerType::Pnpm.update_command(),
            "pnpm update"
        );
    }

    #[test]
    fn test_package_manager_uninstall_command() {
        assert_eq!(
            NodePackageMannegerType::Npm.uninstall_command(),
            "npm uninstall"
        );
        assert_eq!(
            NodePackageMannegerType::Yarn.uninstall_command(),
            "yarn remove"
        );
        assert_eq!(
            NodePackageMannegerType::Pnpm.uninstall_command(),
            "pnpm remove"
        );
    }
}
