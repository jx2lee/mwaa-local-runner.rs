use anyhow::Result;
use std::process::{Command, exit};

use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Build Image Locally")]
pub struct Cli {
    // Build Image Locally
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let build_command = "docker build --rm --compress --platform=linux/amd64 -t private-docker.bithumb.com/amazon/mwaa-local:$AIRFLOW_VERSION";
        let mut command = Command::new(build_command);

        let status = command.status()?;
        exit(status.code().unwrap_or(1));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_image() {}
}
