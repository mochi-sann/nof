use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn execute_command(command: String, args: String) {
    println!("command: {}", command);
    Command::new(command).arg(&"run").arg(&args).exec();
}
