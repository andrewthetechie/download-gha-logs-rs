use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "andrew@💻.kz", version, about, long_about = None)]
pub struct Args {
    /// Repo to download logs from, in the form owner/repo
    #[clap()]
    pub repo: String,

    /// Optional Run IDs to download, can be repeated to download multiple runs. If not set, will lookup runs based on your other options
    #[clap(short, long)]
    pub run_ids: Vec<u64>,

    /// Path to output to. Defaults to .
    #[clap(short, long, default_value = ".")]
    pub output_path: String,

    /// Flag, if set will extract the zip files to the output_path. Defaults to false.
    #[clap(long, takes_value = false)]
    pub extract: bool,

    /// Optional, workflow yaml file name in the repo to get runs from.
    #[clap(short, long)]
    pub workflow_file: Option<String>,

    /// Optional, actor to filter by. If not set, will lookup runs by all actors
    #[clap(short, long)]
    pub actor: Option<String>,

    /// Optional, branch to filter by. If not set, will lookup runs for the repo's default branch
    #[clap(short, long)]
    pub branch: Option<String>,

    /// Optional, event to filter by. If not set, will lookup runs for all events
    #[clap(short, long)]
    pub event: Option<String>,

    /// Optional, status to filter by. If not set, will lookup runs for all statuses
    #[clap(short, long)]
    pub status: Option<String>,

    /// Integer count of the number of runs to return. Default 10
    #[clap(short, long, default_value_t = 10)]
    pub count: u32,

    /// Offset of the runs to return. We start counting runs at the newest run first. An offset of 5 would skip the 5 newest runs when doing a lookip. Default 0
    #[clap(long, default_value_t = 0)]
    pub offset: u32,
}
