use std::os::unix::process::CommandExt;
use std::process::Command;

use super::package_commands::ReturnCoomad;

pub fn execute_command(command: ReturnCoomad) {
    println!("run : {} {} ", command.script, command.args);
    Command::new(command.script).arg(&command.args).exec();
}
