use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

const CLI_LOGO: &str = r#"
______  _____       _______________ 
___   |/  /_ |     / /__    |__    |
__  /|_/ /__ | /| / /__  /| |_  /| |
_  /  / / __ |/ |/ / _  ___ |  ___ |
/_/  /_/  ____/|__/  /_/  |_/_/  |_|

______                    _____________
___  / __________________ ___  /__  __ \___  ____________________________
__  /  _  __ \  ___/  __ `/_  /__  /_/ /  / / /_  __ \_  __ \  _ \_  ___/
_  /___/ /_/ / /__ / /_/ /_  / _  _, _// /_/ /_  / / /  / / /  __/  /
/_____/\____/\___/ \__,_/ /_/  /_/ |_| \__,_/ /_/ /_//_/ /_/\___//_/
"#;

#[derive(Parser, Debug)]
#[command(author, version, bin_name = "mwaa-local-env", disable_help_subcommand = true, about = CLI_LOGO)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[arg(long, global = true, default_value = "2.10.1")]
    pub airflow_version: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    BuildImage(super::build_image::Cli),
    ResetDb(super::reset_db::Cli),
    Start(super::start::Cli),
    TestRequirements(super::test_requirements::Cli),
    PackageRequirements(super::package_requirements::Cli),
    TestStartupScript(super::test_startup_script::Cli),
    ValidatePrereqs(super::validate_prereqs::Cli),
    LoginWeb(super::login_web::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::BuildImage(cli) => cli.exec(),
            Commands::ResetDb(cli) => cli.exec(),
            Commands::Start(cli) => cli.exec(),
            Commands::TestRequirements(cli) => cli.exec(),
            Commands::PackageRequirements(cli) => cli.exec(),
            Commands::TestStartupScript(cli) => cli.exec(),
            Commands::ValidatePrereqs(cli) => cli.exec(),
            Commands::LoginWeb(cli) => cli.exec(),
        }
    }
}
