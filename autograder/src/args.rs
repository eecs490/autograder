use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    /// path to output of running our tests on their solution
    #[structopt(long)]
    our_test_outputs: PathBuf,

    /// path to output of running their tests on our solution
    #[structopt(long)]
    their_test_outputs: PathBuf,

    /// path to submission/Cargo.toml
    #[structopt(long)]
    submission: PathBuf,

    /// path where results.json will be written
    #[structopt(long)]
    output: PathBuf,

    /// path to lcov.info
    #[structopt(long)]
    lcov: PathBuf,

    /// path to scores.yaml
    #[structopt(long)]
    scores: PathBuf,

    /// path to labels.yaml
    #[structopt(long)]
    labels: PathBuf,
}

//impl Opt {
//pub fn get_path_buf(&self, arg: &str) -> Result<PathBuf> {
//let matches: &clap::ArgMatches = &self.0;
//value_t!(matches, arg.clone(), PathBuf).context(Argument { arg })
//}
//}
