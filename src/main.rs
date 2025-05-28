mod commands;
mod files;
mod config;

use clap::Parser;
use commands::{Cli, Commands};

fn main() {
    // TODO: how to use config_path
    let _config_path = match config::find_config() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {e}. Please run `mwaa-runner config` to create one.");
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::BuildImage {} => {
            println!(
                "Building image with Airflow version: {:?}",
                cli.airflow_version
            );
            // TODO
        }
        Commands::Start { project_name } => {
            println!("Starting local Airflow with project name: {}", project_name);
            // TODO
        }
        Commands::ResetDb { project_name } => {
            println!(
                "Resetting local PostgresDB container with project name: {}",
                project_name
            );
            // TODO
        }
        Commands::TestRequirments { skip_build } => {
            println!(
                "Testing requirements with Airflow version: {} and skip_build: {}",
                cli.airflow_version, skip_build
            );
            // TODO
        }
        Commands::PackageRequirements { whl_path } => {
            println!("Packaging requirements with WHL path: {}", whl_path);
            // TODO
        }
    }
}
