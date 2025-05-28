use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Execute shell script on an ephemeral instance of the container")]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_startup_script() {}
}
