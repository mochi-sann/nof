use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};

pub fn execute_command(command: String, args: String) {
    println!("command: {}", command);
    let mut child = Command::new(command)
        .arg(&"run")
        .arg(&args)
        .spawn()
        .expect("failed to start `ls`");
}

