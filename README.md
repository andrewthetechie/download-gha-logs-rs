# download-gha-logs-rs

This project is a Rust CLI app that downloads logs from github actions runs to your local disk, optionally extracting them.

This project uses:

- clap - For CLI
- Octocrab - Github interactions
- tokio - for async
- zip - For extracting zip archives

<p align="center">
    <a href="https://github.com/andrewthetechie/download-gha-logs-rs" target="_blank">
        <img src="https://img.shields.io/github/last-commit/andrewthetechie/download-gha-logs-rs" alt="Latest Commit">
    </a>
    <img src="https://img.shields.io/badge/license-MIT-green">
    <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/andrewthetechie/download-gha-logs-rs?label=Latest%20Release">
    <br />
    <a href="https://github.com/andrewthetechie/download-gha-logs-rs/issues"><img src="https://img.shields.io/github/issues/andrewthetechie/download-gha-logs-rs" /></a>
    <img alt="GitHub Workflow Status Test and Lint (branch)" src="https://img.shields.io/github/workflow/status/andrewthetechie/download-gha-logs-rs/Tests/main?label=Test and Lint">
    <img alt="GitHub Workflow Status Build and Docker (main)" src="https://img.shields.io/github/workflow/status/andrewthetechie/download-gha-logs-rs/Release/main?label=Build and Docker">
    <br />
    <img alt="GitHub all releases" src="https://img.shields.io/github/downloads/andrewthetechie/download-gha-logs-rs/total?color=green">
    <a href='https://hub.docker.com/r/andrewthetechie/download-gha-logs-rs' target="_blank"><img alt="Docker Pulls" src="https://img.shields.io/docker/pulls/andrewthetechie/download-gha-logs-rs">
    <img alt="Docker Image Size (latest by date)" src="https://img.shields.io/docker/image-size/andrewthetechie/download-gha-logs-rs?label=Docker%20Image%20Size"></a>
</p>

## Installation

Download the appropriate binary from a release. Builds are available for Linux, OSX, and Windows in amd64 and arm64 arch.

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
