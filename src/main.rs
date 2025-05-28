mod commands;

use clap::Parser;
use commands::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::BuildImage {
            airflow_version,
            python_version,
        } => {
            println!(
                "Building image with Airflow version: {} and Python version: {}",
                airflow_version, python_version
            );
            // TODO
        }
        Commands::Start {
            project_name,
        } => {
            println!(
                "Starting local Airflow with project name: {}",
                project_name
            );
            // TODO
        }
        Commands::ResetDb { 
            project_name,
        } => {
            println!(
                "Resetting local PostgresDB container with project name: {}",
                project_name
            );
            // TODO
        }
        Commands::TestRequirments { airflow_version, skip_build,} => {
            println!(
                "Testing requirements with Airflow version: {} and skip_build: {}",
                airflow_version, skip_build
            );
            // TODO
        }
    }
}
