use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// path to output of running our tests on their solution
    #[structopt(long)]
    pub our_test_outputs: PathBuf,

    /// path to output of running their tests on our solution
    #[structopt(long)]
    pub their_test_outputs: PathBuf,

    /// path to submission/Cargo.toml
    #[structopt(long)]
    pub submission: PathBuf,

    /// path where results.json will be written
    #[structopt(long)]
    pub output: PathBuf,

    /// path to lcov.info
    #[structopt(long)]
    pub lcov: PathBuf,

    /// path to scores.yaml
    #[structopt(long)]
    pub scores: PathBuf,

    /// path to labels.yaml
    #[structopt(long)]
    pub labels: PathBuf,
}
