use anyhow::Result;
use clap::Args;
use std::{
    io::{self, Write},
    path::PathBuf,
};
use which::which;

#[derive(Args, Debug)]
#[command(about = "Validate pre-reqs installed (docker, docker-compose, python3, pip3)")]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let required_commands = ["docker", "docker-compose", "python3", "pip3"];
        let mut stdout = io::BufWriter::new(io::stdout());

        for command in required_commands {
            match check_command(command) {
                Some(path) => writeln!(stdout, "{} is Installed. ✔ {}", command, path.display())?,
                None => writeln!(
                    stdout,
                    "{} is not installed or not runnable without sudo.",
                    command
                )?,
            }
        }
        Ok(())
    }
}

fn check_command(cmd: &str) -> Option<PathBuf> {
    which(cmd).ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_command_when_missing_and_existing_command() {
        assert!(crate::cmd::validate_prereqs::check_command("some-nonexistent-command").is_none());
        assert!(crate::cmd::validate_prereqs::check_command("ls").is_some());
    }
}
