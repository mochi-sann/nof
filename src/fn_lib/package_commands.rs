use std::process::Command;

use crate::debug;

use super::command_list::{NpmCoomands, COMMAND_LIST};

#[derive(Debug, PartialEq, clap::ValueEnum, Clone)]
pub enum NodePackageMannegerType {
    Npm,
    Yarn,
    Pnpm,
    Bun,
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
            NodePackageMannegerType::Bun => COMMAND_LIST.bun,
        }
    }
    pub fn is_installed_command(&self) -> bool {
        let output = Command::new("which")
            .arg(self.get_commands().command_name)
            .output()
            .expect("failed to execute process");

        output.status.success()
    }

    pub fn run_node_scripts(&self, scripts: String) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = self.get_commands().command_name;
        command_args.push(self.get_commands().run.to_string());
        command_args.push(scripts);

        debug!(package_script);
        ReturnCoomad {
            script: package_script.to_string(),
            args: command_args,
        }
    }

    pub fn install_command(
        &self,
        save_dev: bool,
        save_peer: bool,
        save_optional: bool,
        frozen_lockfile: bool,
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
        match frozen_lockfile {
            true => command_args.push(self.get_commands().frozen_lockfile.to_string()),
            false => {}
        }

        ReturnCoomad {
            script: package_script.to_string(),
            args: command_args,
        }
    }
    pub fn add(
        &self,
        lib: Vec<String>,
        save_dev: bool,
        save_peer: bool,
        save_optional: bool,
    ) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = self.get_commands().command_name;
        let add_command = self.get_commands().add;
        command_args.push(add_command.to_string());
        // command_args.push(package_name);
        debug!(&lib);
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

        for package in &lib {
            command_args.push(package.to_string());
        }

        ReturnCoomad {
            script: package_script.to_string(),
            args: command_args,
        }
    }
    pub fn remove(&self, lib: Vec<String>) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        let package_script = self.get_commands().command_name;
        command_args.push(self.get_commands().remove.to_string());
        // command_args.push(package_name);
        debug!(&lib);

        for package in &lib {
            command_args.push(package.to_string());
        }

        ReturnCoomad {
            script: package_script.to_string(),
            args: command_args,
        }
    }
    pub fn execute_command(&self, command: &Vec<String>) -> ReturnCoomad {
        let mut command_args: Vec<String> = vec![];
        for i in command {
            command_args.push(i.to_string());
        }
        ReturnCoomad {
            script: self.get_commands().execute_command.to_string(),
            args: command_args,
        }
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
                package_manager.install_command(false, false, false, false),
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
                package_manager.install_command(false, false, false, false),
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
                package_manager.install_command(false, false, false, false),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
        // add bun test for install_command test
        #[test]
        fn test_package_manager_install_command_bun() {
            let package_manager = NodePackageMannegerType::Bun;
            assert_eq!(
                package_manager.install_command(false, false, false, false),
                ReturnCoomad {
                    script: "bun".to_string(),
                    args: vec!["install".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_install_command_lib_none_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(false, false, false, false),
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
                package_manager.install_command(false, false, false, false),
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
                package_manager.install_command(false, false, false, false),
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
                package_manager.install_command(true, false, false, false),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["install".to_string(), "--save-dev".to_string(),],
                }
            );
        }
        #[test]
        fn test_package_manager_install_frozen_install_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.install_command(false, false, false, true),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["install".to_string(), "--frozen-lockfile".to_string(),],
                }
            );
        }
        #[test]
        fn test_package_manager_install_save_dev_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.install_command(true, false, false, false),
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
                package_manager.install_command(true, false, false, false),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["install".to_string(), "--save-dev".to_string(),],
                }
            );
        }
    }
    mod add {
        use crate::fn_lib::package_commands::{NodePackageMannegerType, ReturnCoomad};

        #[test]
        fn test_package_manager_add_command_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.add(
                    vec!["hoge".to_string(), "fuga".to_string()],
                    false,
                    false,
                    false,
                ),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec![
                        "install".to_string(),
                        "hoge".to_string(),
                        "fuga".to_string()
                    ],
                }
            );
        }
        #[test]
        fn test_package_manager_add_command_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.add(
                    vec!["hoge".to_string(), "fuga".to_string()],
                    false,
                    false,
                    false,
                ),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["add".to_string(), "hoge".to_string(), "fuga".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_add_command_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.add(
                    vec!["hoge".to_string(), "fuga".to_string()],
                    false,
                    false,
                    false,
                ),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["add".to_string(), "hoge".to_string(), "fuga".to_string()],
                }
            );
        }
        #[test]
        fn test_package_manager_add_command_pnpm_save_dev() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.add(
                    vec!["hoge".to_string(), "fuga".to_string()],
                    true,
                    false,
                    false,
                ),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec![
                        "add".to_string(),
                        "--save-dev".to_string(),
                        "hoge".to_string(),
                        "fuga".to_string()
                    ],
                }
            );
        }
        #[test]
        fn test_package_manager_add_command_pnpm_save_peer() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.add(
                    vec!["hoge".to_string(), "fuga".to_string()],
                    false,
                    true,
                    false,
                ),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec![
                        "add".to_string(),
                        "--save-peer".to_string(),
                        "hoge".to_string(),
                        "fuga".to_string()
                    ],
                }
            );
        }
    }
    mod remove {
        use crate::fn_lib::package_commands::{NodePackageMannegerType, ReturnCoomad};

        #[test]
        fn remove_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.remove(vec!["hoge".to_string(), "test".to_string()]),
                ReturnCoomad {
                    script: "pnpm".to_string(),
                    args: vec!["remove".to_string(), "hoge".to_string(), "test".to_string()],
                }
            );
        }
        #[test]
        fn remove_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.remove(vec!["hoge".to_string(), "test".to_string()]),
                ReturnCoomad {
                    script: "npm".to_string(),
                    args: vec!["remove".to_string(), "hoge".to_string(), "test".to_string()],
                }
            );
        }
        #[test]
        fn remove_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.remove(vec!["hoge".to_string(), "test".to_string()]),
                ReturnCoomad {
                    script: "yarn".to_string(),
                    args: vec!["remove".to_string(), "hoge".to_string(), "test".to_string()],
                }
            );
        }
    }

    mod execute_command {
        use crate::fn_lib::package_commands::{NodePackageMannegerType, ReturnCoomad};

        #[test]
        fn execute_npm() {
            let package_manager = NodePackageMannegerType::Npm;
            assert_eq!(
                package_manager.execute_command(&vec!["test".to_string()]),
                ReturnCoomad {
                    script: "npx".to_string(),
                    args: vec!["test".to_string()],
                }
            );
        }

        #[test]
        fn execute_yarn() {
            let package_manager = NodePackageMannegerType::Yarn;
            assert_eq!(
                package_manager.execute_command(&vec!["test".to_string()]),
                ReturnCoomad {
                    script: "yarn -s run".to_string(),
                    args: vec!["test".to_string()],
                }
            );
        }
        #[test]
        fn execute_pnpm() {
            let package_manager = NodePackageMannegerType::Pnpm;
            assert_eq!(
                package_manager.execute_command(&vec!["test".to_string()]),
                ReturnCoomad {
                    script: "pnpx".to_string(),
                    args: vec!["test".to_string()],
                }
            );
        }
    }
}
