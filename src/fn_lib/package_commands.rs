#[derive(Debug, PartialEq, clap::ValueEnum, Clone)]
pub enum NodePackageMannegerType {
    Npm,
    Yarn,
    Pnpm,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnCoomad {
    pub script: String,
    pub args: Vec<String>,
}

impl NodePackageMannegerType {
    pub fn run_node_scripts(&self, scripts: String) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = match self {
            NodePackageMannegerType::Npm => "npm",
            NodePackageMannegerType::Yarn => "yarn",
            NodePackageMannegerType::Pnpm => "pnpm",
        };
        command_args.push("run".to_string());
        command_args.push(scripts);

        return ReturnCoomad {
            script: format!("{}", package_script.to_string()),
            args: command_args,
        };
    }
    pub fn install_command(&self, lib: Option<String>) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = match self {
            NodePackageMannegerType::Npm => "npm",
            NodePackageMannegerType::Yarn => "yarn",
            NodePackageMannegerType::Pnpm => "pnpm",
        };
        let install_command = match self {
            NodePackageMannegerType::Npm => "install",
            NodePackageMannegerType::Yarn => match lib {
                None => "install",
                _ => "add",
            },
            NodePackageMannegerType::Pnpm => "install",
        };
        command_args.push(install_command.to_string());

        match lib {
            Some(v) => {
                command_args.push(v);
            }
            None => {}
        }
        return ReturnCoomad {
            script: package_script.to_string(),
            args: command_args,
        };
    }
}

#[cfg(test)]
mod tests {
    mod run_node_scripts {
        use crate::fn_lib::package_commands::{NodePackageMannegerType, ReturnCoomad};

        #[test]
        fn test_package_manager_run_command_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.run_node_scripts("test".to_string()),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["run".to_string(), "test".to_string()],
                }
            );
        }
        // add test for install_command

        #[test]
        fn test_package_manager_run_command_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.run_node_scripts("test".to_string()),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["run".to_string(), "test".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_run_command_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.run_node_scripts("test".to_string()),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["run".to_string(), "test".to_string()],
                }
            );
        }
    }
    mod install {
        use crate::fn_lib::package_commands::{NodePackageMannegerType, ReturnCoomad};

        #[test]
        fn test_package_manager_install_command_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.install_command(Some("test".to_string())),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["install".to_string(), "test".to_string()],
                }
            );
        }

        // add yarn and pnpm test for install_command test
        #[test]
        fn test_package_manager_install_command_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(Some("test".to_string())),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["add".to_string(), "test".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_command_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.install_command(Some("test".to_string())),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string(), "test".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_command_lib_none_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(None),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }

        #[test]
        fn test_package_manager_install_command_lib_none_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.install_command(None),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_command_lib_none_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.install_command(None),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
    }
}
