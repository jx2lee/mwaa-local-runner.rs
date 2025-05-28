use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Reset local PostgresDB container")]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reset_db() {}
}
