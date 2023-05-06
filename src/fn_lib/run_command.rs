use std::process::Command;

use termion::cursor::Show;

use crate::debug;

use super::package_commands::ReturnCoomad;

pub fn execute_command(command: ReturnCoomad) {
    debug!(command.clone());

    Command::new("clear").status().unwrap();
    // redisplay the cursor
    print!("{}", Show);
    Command::new(command.script)
        .args(command.args)
        .status()
        .unwrap();
}
