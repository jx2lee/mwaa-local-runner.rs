use crate::files::find_requirements_dir;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true, default_value = "2.10.3")]
    pub airflow_version: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build a new image with the current requriements.txt
    BuildImage {},

    /// Reset local PostgresDB container.
    ResetDb {
        /// Docker compose project name
        #[arg(long, default_value = "aws-mwaa-local")]
        project_name: String,
    },

    /// Start the local Airflow
    Start {
        /// Docker compose project name
        #[arg(long, default_value = "aws-mwaa-local-runner")]
        project_name: String,
    },

    /// Install requirements on an ephemeral instance of the container.
    TestRequirments {
        /// Skip building the image
        #[arg(long, default_value = "false")]
        skip_build: bool,
    },

    /// Download requirements WHL files into plugins folder.
    PackageRequirements {
        /// path for WHL files
        #[arg(long, default_value_t = find_requirements_dir())]
        whl_path: String,
    },
}
