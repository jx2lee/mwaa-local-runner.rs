use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Download requirements WHL files into plugins folder")]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_package_requirements() {}
}
