use std::process::Command;
use std::io::{self, Write};

use termion::cursor::Show;
use termion::color;

use crate::debug;

use super::package_commands::{ReturnCoomad, NodePackageMannegerType};

pub fn show_suggestions(suggestions: Vec<String>) {
    if !suggestions.is_empty() {
        println!("\n{}Suggestions:{}", color::Fg(color::Blue), color::Fg(color::Reset));
        for (i, suggestion) in suggestions.iter().enumerate() {
            println!("{}. {}", i + 1, suggestion);
        }
    }
}

pub fn execute_command(package_manager: NodePackageMannegerType, command: ReturnCoomad) {
    debug!(command.clone());

    // Show command suggestions before executing
    if !command.args.is_empty() {
        let input = command.args[0].clone();
        let suggestions = package_manager.get_command_suggestions(&input);
        show_suggestions(suggestions);
    }

    Command::new("clear").status().unwrap();
    // redisplay the cursor
    print!("{}", Show);
    Command::new(command.script)
        .args(command.args)
        .status()
        .unwrap();
}
