mod fn_lib;
mod fzf_scripts;
mod read_package_json;

use clap::Parser;
use fn_lib::{
    get_directory_from_file_path::get_directory_from_file_path,
    package_commands::{NodePackageMannegerType, ReturnCoomad},
    run_command::execute_command,
    type_node_pac::detect_package_manager,
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
    #[command(about = "Run node scripts" , visible_aliases = [ "r" , "R" , "run-script"])]
    Run {
        #[arg(short, long, default_value = "./package.json")]
        target_path: String,

        #[arg(short, long, value_enum)]
        package_manneger: Option<NodePackageMannegerType>,

        #[arg(short, long)]
        script: Option<String>,
    },
    #[command(about = "Installs all dependencies", visible_aliases = [ "i" , "I" ])]
    Install {
        #[arg(short, long, default_value = "./package.json")]
        target_path: String,

        #[arg(short, long, value_enum, help = "Specify the package manager")]
        package_manneger: Option<NodePackageMannegerType>,

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
        #[arg(short, long, default_value = "./package.json")]
        target_path: String,
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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run {
            target_path,
            package_manneger: package_manneger_select,
            script: command_scipts,
        } => {
            let folder_path = get_directory_from_file_path(&target_path);
            let scripts_list = get_scripts(target_path.to_string());

            let script = match command_scipts {
                None => fzf_scripts::fzf_scipts(scripts_list),
                Some(v) => vec![v.clone()],
            };

            let package_manager = match package_manneger_select {
                None => detect_package_manager(&folder_path.expect("./").to_str().unwrap()),
                Some(v) => v.clone(),
            };
            let run_scripts = package_manager.run_node_scripts(script[0].to_string());

            execute_command(run_scripts);
        }
        Commands::Install {
            target_path,
            package_manneger,
            save_dev,
            save_peer,
            save_optional,
        } => {
            let folder_path = get_directory_from_file_path(&target_path);
            let package_manager = match package_manneger {
                None => detect_package_manager(&folder_path.expect("./").to_str().unwrap()),
                Some(v) => v.clone(),
            };
            let install_command = package_manager.install_command(
                save_dev.clone(),
                save_peer.clone(),
                save_optional.clone(),
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
            let folder_path = get_directory_from_file_path(&target_path);
            let package_manager = match package_manneger {
                None => detect_package_manager(&folder_path.expect("./").to_str().unwrap()),
                Some(v) => v.clone(),
            };
            let install_command =
                package_manager.add(lib.to_vec(), *save_dev, *save_peer, *save_optional);
            debug!(install_command.clone());
        }
    }
    // get_scripts();
}
