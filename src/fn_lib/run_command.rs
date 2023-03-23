use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::debug;

use super::package_commands::ReturnCoomad;

pub fn execute_command(command: ReturnCoomad) {
    println!("run : {} {} ", command.script, command.args);
    debug!(command.clone());

    Command::new(command.script).arg(&command.args).exec();
}
