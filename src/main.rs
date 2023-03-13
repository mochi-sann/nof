mod fn_lib;
mod fzf_scripts;
mod read_package_json;
mod select_node_package;

use clap::Parser;
use fn_lib::{
    get_directory_from_file_path::get_directory_from_file_path,
    type_node_pac::{detect_package_manager, NodePackageMannegerType},
};
use read_package_json::get_scripts;

use crate::fn_lib::run_node_scripts::run_node_scripts;

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
    #[command(about = "Run node scripts")]
    Run {
        #[arg(short, long, default_value = "./package.json")]
        target_path: String,

        #[arg(short, long, value_enum)]
        package_manneger: Option<NodePackageMannegerType>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run {
            target_path,
            package_manneger: package_manneger_select,
        } => {
            let folder_path = get_directory_from_file_path(&target_path);
            let scripts_list = get_scripts(target_path.to_string());
            let script = fzf_scripts::fzf_scipts(scripts_list);

            let package_manager = match package_manneger_select {
                None => detect_package_manager(&folder_path.expect("./").to_str().unwrap()),
                Some(v) => v.clone(),
            };
            run_node_scripts(package_manager, script[0].to_string());
        }
    }
    // get_scripts();
}
