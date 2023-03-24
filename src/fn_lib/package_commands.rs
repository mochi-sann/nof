use crate::debug;

use super::command_list::{NpmCoomands, COMMAND_LIST};

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
    pub fn get_commands(&self) -> NpmCoomands {
        match self {
            NodePackageMannegerType::Yarn => COMMAND_LIST.yarn,
            NodePackageMannegerType::Npm => COMMAND_LIST.npm,
            NodePackageMannegerType::Pnpm => COMMAND_LIST.pnpm,
        }
    }

    pub fn run_node_scripts(&self, scripts: String) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = self.get_commands().command_name;
        command_args.push("run".to_string());
        command_args.push(scripts);

        debug!(package_script);
        return ReturnCoomad {
            script: format!("{}", package_script.to_string()),
            args: command_args,
        };
    }

    pub fn install_command(
        &self,
        save_dev: bool,
        save_peer: bool,
        save_optional: bool,
    ) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = self.get_commands().command_name;
        let install_command = self.get_commands().isntall;
        command_args.push(install_command.to_string());

        match save_dev {
            true => command_args.push(self.get_commands().save_dev.to_string()),
            false => {}
        }
        match save_peer {
            true => command_args.push(self.get_commands().save_peer.to_string()),
            false => {}
        }
        match save_optional {
            true => command_args.push(self.get_commands().save_optional.to_string()),
            false => {}
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
                package_manager.install_command(false, false, false),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }

        // add yarn and pnpm test for install_command test
        #[test]
        fn test_package_manager_install_command_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(false, false, false),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_command_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.install_command(false, false, false),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_command_lib_none_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(false, false, false),
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
                package_manager.install_command(false, false, false),
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
                package_manager.install_command(false, false, false),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
    }
    mod save_optional {
        use crate::fn_lib::package_commands::{NodePackageMannegerType, ReturnCoomad};

        #[test]
        fn test_package_manager_install_save_dev_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.install_command(true, false, false),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["install".to_string(), "--save-dev".to_string(),],
                }
            );
        }
        #[test]
        fn test_package_manager_install_save_dev_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(true, false, false),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["install".to_string(), "-D".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_save_dev_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.install_command(true, false, false),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string(), "--save-dev".to_string(),],
                }
            );
        }
    }
}
