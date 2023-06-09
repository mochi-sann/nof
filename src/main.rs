mod fn_lib;
mod fzf_scripts;
mod read_package_json;

use std::{io, path::PathBuf};

use clap::{Command, CommandFactory, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};
use fn_lib::{
    get_directory_from_file_path::get_directory_from_file_path,
    match_package_maneger::check_installde_package_maneger,
    package_commands::{NodePackageMannegerType, ReturnCoomad},
    run_command::execute_command,
};
use read_package_json::get_scripts;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    #[command(about = "Generates a script for completion")]
    Completion {
        #[clap(long, short, value_enum)]
        shell: Shell,
    },

    #[command(about = "Run node scripts" , visible_aliases = [ "r" , "R" , "run-script"])]
    Run {
        #[arg(short, long, default_value = "./package.json", value_hint = ValueHint::FilePath)]
        target_path: PathBuf,

        #[arg(short, long, value_enum)]
        package_manneger: Option<NodePackageMannegerType>,

        // run script
        script: Option<String>,
    },
    #[command(about = "Installs all dependencies", visible_aliases = [ "i" , "I" ])]
    Install {
        #[arg(short, long, default_value = "./package.json" , value_hint = ValueHint::FilePath)]
        target_path: PathBuf,

        #[arg(short, long, value_enum, help = "Specify the package manager")]
        package_manneger: Option<NodePackageMannegerType>,

        #[arg(
            short = 'F',
            long,
            default_value = "false",
            help = "don't generate a lockfile and fail if an update is needed"
        )]
        frozen_lockfile: bool,

        #[arg(
            short = 'D',
            long,
            default_value = "false",
            help = "save package to your `devDependencies`"
        )]
        save_dev: bool,
        #[arg(
            short = 'P',
            long,
            default_value = "false",
            help = "save package to your `peerDependencies`"
        )]
        save_peer: bool,
        #[arg(
            short = 'O',
            long,
            default_value = "false",
            help = "save package to your `optionalDependencies`"
        )]
        save_optional: bool,
    },

    #[command(about = "Installs a package", visible_aliases = [ "a" , "A" ])]
    Add {
        #[arg(short, long, default_value = "./package.json" , value_hint = ValueHint::FilePath)]
        target_path: PathBuf,
        #[arg(short, long, value_enum, help = "Specify the package manager")]
        package_manneger: Option<NodePackageMannegerType>,

        /// Name of package to install
        packages: Vec<String>,

        #[arg(
            short = 'D',
            long,
            default_value = "false",
            help = "save package to your `devDependencies`"
        )]
        save_dev: bool,
        #[arg(
            short = 'P',
            long,
            default_value = "false",
            help = "save package to your `peerDependencies`"
        )]
        save_peer: bool,
        #[arg(
            short = 'O',
            long,
            default_value = "false",
            help = "save package to your `optionalDependencies`"
        )]
        save_optional: bool,
    },
    #[command(about = "remove a package", visible_aliases =  [ "rm" ] )]
    Remove {
        /// Name of package to remove
        packages: Vec<String>,

        #[arg(short, long, default_value = "./package.json" , value_hint = ValueHint::FilePath)]
        target_path: PathBuf,
        #[arg(short, long, value_enum, help = "Specify the package manager")]
        package_manneger: Option<NodePackageMannegerType>,
    },

    #[command(about = "Run a command from a local or remote npm package", visible_aliases =  [ "e" , "exec"  ,"E"  ] )]
    ExecuteCommand {
        #[arg(short, long, default_value = "./package.json" , value_hint = ValueHint::FilePath)]
        target_path: PathBuf,
        #[arg(short, long, value_enum, help = "Specify the package manager")]
        package_manneger: Option<NodePackageMannegerType>,

        /// Name of package to run
        packages: Vec<String>,
    },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            print_completions(*shell, &mut cmd);
        }
        Commands::Run {
            target_path,
            package_manneger: package_manneger_select,
            script: command_scipts,
        } => {
            let folder_path = get_directory_from_file_path(target_path);
            let scripts_list = get_scripts(target_path);

            let script = match command_scipts {
                None => fzf_scripts::fzf_scipts(scripts_list),
                Some(v) => vec![v.clone()],
            };

            let package_manager =
                check_installde_package_maneger(package_manneger_select, folder_path);
            let run_scripts = package_manager.run_node_scripts(script[0].to_string());

            execute_command(run_scripts);
        }
        Commands::Install {
            target_path,
            package_manneger,
            save_dev,
            save_peer,
            save_optional,
            frozen_lockfile,
        } => {
            let folder_path = get_directory_from_file_path(target_path);
            let package_manager = check_installde_package_maneger(package_manneger, folder_path);

            let install_command = package_manager.install_command(
                *save_dev,
                *save_peer,
                *save_optional,
                *frozen_lockfile,
            );
            debug!(install_command.clone());
            debug!(package_manneger);
            debug!(save_dev);
            debug!(install_command.clone());

            let run_script: ReturnCoomad = install_command;
            execute_command(run_script);
        }
        Commands::Add {
            target_path,
            packages: lib,
            save_dev,
            save_optional,
            save_peer,
            package_manneger,
        } => {
            let folder_path = get_directory_from_file_path(target_path);
            let package_manager = check_installde_package_maneger(package_manneger, folder_path);
            let add_command =
                package_manager.add(lib.to_vec(), *save_dev, *save_peer, *save_optional);
            debug!(add_command.clone());
            execute_command(add_command);
        }
        Commands::Remove {
            packages,
            package_manneger,
            target_path,
        } => {
            let folder_path = get_directory_from_file_path(target_path);
            let package_manager = check_installde_package_maneger(package_manneger, folder_path);
            let install_command = package_manager.remove(packages.to_vec());
            debug!(install_command.clone());
            debug!(package_manneger);
            debug!(install_command.clone());

            let run_script: ReturnCoomad = install_command;
            execute_command(run_script);
        }
        Commands::ExecuteCommand {
            target_path,
            package_manneger,
            packages,
        } => {
            let folder_path = get_directory_from_file_path(target_path);
            let package_manager = check_installde_package_maneger(package_manneger, folder_path);
            let run_script = package_manager.execute_command(&packages.to_vec());
            execute_command(run_script);
        }
    }
    // get_scripts();
}
