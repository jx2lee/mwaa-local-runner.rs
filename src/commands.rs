use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build a new image with the current requriements.txt
    BuildImage {
        /// Airflow version to use
        #[arg(long, default_value = "2.10.3")]
        airflow_version: String,

        /// Python version touse
        #[arg(long, default_value = "3.10")]
        python_version: String,
    },

    /// Reset local PostgresDB container.
    ResetDb {
        /// Docker compose project name
        #[arg(long, default_value = "aws-mwaa-local")]
        project_name: String,
    },
    
    /// Start the local Airflow
    Start {
        /// Docker compose project name
        #[arg(long, default_value = "aws-mwaa-local")]
        project_name: String,
    },

    /// Install requirements on an ephemeral instance of the container.
    TestRequirments {
        /// Airflow version to use
        #[arg(long, default_value = "2.10.3")]
        airflow_version: String,

        /// Skip building the image
        #[arg(long, default_value = "false")]
        skip_build: bool,
    }
}
