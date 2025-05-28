#[macro_use]
mod macros;
mod app;
mod cmd;

use anyhow::Result;
use clap::Parser;

use crate::cmd::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    app::set_global_verbosity(cli.verbose.log_level_filter());

    cli.exec()
}
