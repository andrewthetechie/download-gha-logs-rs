# download-gha-logs-rs

This project is a Rust CLI app that downloads logs from github actions runs to your local disk, optionally extracting them.

This project uses:

- clap - For CLI
- Octocrab - Github interactions
- tokio - for async
- zip - For extracting zip archives

## Usage

Set GITHUB_TOKEN to a Github Personal Access token with access to the repo you are trying to download.

```shell
✗ download-gha-logs --help
download-gha-logs 0.1.0
Andrew Herrington <andrew.the.techie@gmail.com>
Download the logs of a Github Actions Run to a local file

USAGE:
    download-gha-logs [OPTIONS] <REPO> [RUN_IDS]...

ARGS:
    <REPO>          Repo to download logs from, in the form owner/repo
    <RUN_IDS>...    Run IDs to download

OPTIONS:
        --extract                      Flag, if set will extract the zip files to the output_path.
                                       Defaults to false
    -h, --help                         Print help information
    -o, --output-path <OUTPUT_PATH>    Path to output to. Defaults to ./ [default: ./]
    -V, --version                      Print version information
```

## Development

The project supports the use of [tokio-rs-console] for debugging/tracing `async` scenarios. Just
enable the `tokio-console` feature.

[tokio-rs-console]: https://github.com/tokio-rs/console
