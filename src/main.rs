mod fn_lib;
mod fzf_scripts;
mod read_package_json;
mod select_node_package;
use clap::Parser;
use fn_lib::type_node_pac::NodePackageMannegerType;
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
    Run {
        #[arg(short, long, default_value = "./package.json")]
        target_path: String,

        #[arg(short, long)]
        package_manneger: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run {
            target_path,
            package_manneger,
        } => {
            // println!("target_path: {:?}", target_path);
            let scripts_list = get_scripts(target_path.to_string());
            // println!("scripts_list: {:?}", scripts_list);
            let script = fzf_scripts::fzf_scipts(scripts_list);
            // println!("script: {:?} ", script);
            let node_package_manneger_type = match package_manneger.as_str() {
                "yarn" => NodePackageMannegerType::Yarn,
                "npm" => NodePackageMannegerType::Npm,
                "pnpm" => NodePackageMannegerType::Pnpm,
                _ => NodePackageMannegerType::Yarn,
            };
        }
    }
    // get_scripts();
}
