use std::{env, io, path::Path};

pub fn find_config() -> Result<String, io::Error> {
    if Path::new(".mwaa-runner.config").exists() {
        return Ok(".mwaa-runner.config".to_string());
    }
    
    let home = env::var("HOME").map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not set"))?;
    let home_config = format!("{}/.mwaa-runner.config", home);

    if Path::new(&home_config).exists() {
        return Ok(home_config);
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No config file found"))
    }
}
