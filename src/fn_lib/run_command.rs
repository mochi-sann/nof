use std::process::Command;

use crate::debug;

use super::package_commands::ReturnCoomad;

pub fn execute_command(command: ReturnCoomad) {
    debug!(command.clone());

    std::process::Command::new("clear").status().unwrap();
    Command::new(command.script).args(command.args).status().unwrap();
}
