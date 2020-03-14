mod error;
use error::Error::{OpenConfig, SaveConfig, UserIdInvalid};
use error::Result;
use snafu::{ensure, ErrorCompat, ResultExt};
use std::{fs, path::Path};

fn log_in_user<P>(config_root: P, user_id: i32) -> Result<bool>
where
    P: AsRef<Path>,
{
    let config_root = config_root.as_ref();
    let filename = &config_root.join("config.toml");

    let config = fs::read(filename).context(OpenConfig { filename })?;
    // Perform updates to config
    fs::write(filename, config).context(SaveConfig { filename })?;

    ensure!(user_id == 42, UserIdInvalid { user_id });

    Ok(true)
}

fn main() {
    match log_in_user("config", 0) {
        Ok(true) => println!("Logged in!"),
        Ok(false) => println!("Not logged in!"),
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                println!("{}", backtrace);
            }
        }
    }
}
