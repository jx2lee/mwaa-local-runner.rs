use std::process::Command;

use anyhow::{Context, Result, anyhow};
use aws_config::{BehaviorVersion, Region};
use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Create MWAA Web URL to explore")]
pub struct Cli {
    #[arg(short = 'e', long, required = true)]
    mwaa_env: String,

    #[arg(short = 'p', long)]
    aws_profile: Option<String>,

    #[arg(long, requires = "aws_profile")]
    with_sso: bool,

    #[arg(short = 'r', long, default_value = "ap-northeast-2")]
    aws_region: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        // load aws config
        debug!("{:#?}", &self);

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        // If user requests SSO within this CLI run, perform login first.
        if self.with_sso {
            let profile = self
                .aws_profile
                .as_ref()
                .ok_or_else(|| anyhow!("Always Use --with-sso with --aws-profile"))?;
            // Trigger interactive device flow in user's browser via aws-cli v2
            sso_login(profile)?;
        }

        // Let AWS SDK resolve credentials. --aws-profile selects named profile;
        // without it, default provider chain handles env vars, default profiles,
        // cached SSO credentials, and instance/task roles.
        let config = runtime.block_on({
            let mut loader = aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new(self.aws_region.clone()));

            if let Some(profile) = &self.aws_profile {
                loader = loader.profile_name(profile);
            }
            loader = loader.region(Region::new(self.aws_region.clone()));
            loader.load()
        });

        let mwaa_client = aws_sdk_mwaa::Client::new(&config);

        // create token
        let token_response = runtime.block_on({
            mwaa_client
                .create_web_login_token()
                .name(&self.mwaa_env)
                .send()
        })
        .with_context(|| "An authentication error occurred while calling CreateWebLoginToken. Please check --with-sso or your credential configuration.")?;

        // Create login url
        let mwaa_web_url = format!(
            "https://{}/aws_mwaa/aws-console-sso?login=true#{}",
            token_response.web_server_hostname.unwrap(),
            token_response.web_token.unwrap(),
        );
        debug!("{:#}", mwaa_web_url);

        // Open browser
        if cfg!(target_os = "macos") {
            Command::new("open")
                .arg("-a")
                .arg("Google Chrome")
                .arg(&mwaa_web_url)
                .output()?;
        } else {
            // macos or windows
            Command::new("chrome.exe").arg(&mwaa_web_url).output()?;
        };

        Ok(())
    }
}

#[allow(dead_code)]
fn sso_login(profile: &str) -> Result<()> {
    let status = Command::new("aws")
        .arg("sso")
        .arg("login")
        .arg("--profile")
        .arg(profile)
        .status()
        .context("failed to spawn `aws sso login`")?;
    if !status.success() {
        return Err(anyhow!("aws sso login failed with profile, {}", &profile));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_login_web() {}
}
