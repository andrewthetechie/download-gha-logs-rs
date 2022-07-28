use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Repo to download logs from, in the form owner/repo
    #[clap()]
    pub repo: String,

    /// Run IDs to download
    #[clap()]
    pub run_ids: Vec<u64>,

    /// Path to output to. Defaults to ./
    #[clap(short, long, default_value = "./")]
    pub output_path: String,

    /// Flag, if set will extract the zip files to the output_path. Defaults to false.
    #[clap(long, takes_value = false)]
    pub extract: bool,
}
