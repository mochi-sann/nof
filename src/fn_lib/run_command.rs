use std::{
    io::{stdout, Write},
    process::Command,
};

use termion::cursor::Show;

use crate::debug;

use super::package_commands::ReturnCoomad;

pub fn execute_command(command: ReturnCoomad) {
    debug!(command.clone());

    Command::new("clear").status().unwrap();
    // redisplay the cursor
    write!(stdout(), "{}", Show).unwrap();
    Command::new(command.script)
        .args(command.args)
        .status()
        .unwrap();
}
