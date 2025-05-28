use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Start Airflow local environment. (LocalExecutor, Using postgres DB)")]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_start() {}
}
