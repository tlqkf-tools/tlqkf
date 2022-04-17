mod constants;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[allow(non_camel_case_types)]
    /// 도움말을 출력합니다
    ehdna,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ehdna => {
            constants::print_korean_help_message();
        }
    }
}
