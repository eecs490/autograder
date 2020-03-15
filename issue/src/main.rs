#![feature(try_trait)]
use snafu;
use snafu::OptionExt;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum MyError {
    #[snafu()]
    NoneError { source: snafu::NoneError },
}

fn main() -> Result<(), MyError> {
    None.context(NoneError {})?;
    Ok(())
}
