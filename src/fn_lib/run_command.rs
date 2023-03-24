use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::debug;

use super::package_commands::ReturnCoomad;

pub fn execute_command(command: ReturnCoomad) {
    debug!(command.clone());
    Command::new(command.script).args(command.args).exec();
}
