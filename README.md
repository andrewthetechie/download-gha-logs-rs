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
andrew@💻.kz
Download the logs of a Github Actions Run to a local file

USAGE:
    download-gha-logs [OPTIONS] <REPO>

ARGS:
    <REPO>    Repo to download logs from, in the form owner/repo

OPTIONS:
    -a, --actor <ACTOR>
            Optional, actor to filter by. If not set, will lookup runs by all actors

    -b, --branch <BRANCH>
            Optional, branch to filter by. If not set, will lookup runs for the repo's default
            branch

    -c, --count <COUNT>
            Integer count of the number of runs to return. Default 10 [default: 10]

    -e, --event <EVENT>
            Optional, event to filter by. If not set, will lookup runs for all events

        --extract
            Flag, if set will extract the zip files to the output_path. Defaults to false

    -h, --help
            Print help information

    -o, --output-path <OUTPUT_PATH>
            Path to output to. Defaults to  [default: .]

        --offset <OFFSET>
            Offset of the runs to return. We start counting runs at the newest run first. An offset
            of 5 would skip the 5 newest runs when doing a lookip. Default 0 [default: 0]

    -r, --run-ids <RUN_IDS>
            Optional Run IDs to download, can be repeated to download multiple runs. If not set,
            will lookup runs based on your other options

    -s, --status <STATUS>
            Optional, status to filter by. If not set, will lookup runs for all statuses

    -V, --version
            Print version information

    -w, --workflow-file <WORKFLOW_FILE>
            Optional, workflow yaml file name in the repo to get runs from
```

### Download a single run

```shell
download-gha-logs-rs --run-ids 2766205906`
2022-07-31T21:52:19.876406Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205906
2022-07-31T21:52:21.210990Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205906.zip
```

### Download and extract a single run

```shell
download-gha-logs-rs --run-ids 2766205906 --extract`
2022-07-31T21:52:44.126999Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205906
2022-07-31T21:52:45.170110Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205906.zip
2022-07-31T21:52:45.176493Z  INFO download_gha_logs: File 0 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/10_YAML Lint.txt" (1875 bytes)
2022-07-31T21:52:45.178516Z  INFO download_gha_logs: File 1 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/20_Post Check out the repository.txt" (1460 bytes)
2022-07-31T21:52:45.180070Z  INFO download_gha_logs: File 2 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/4_Run Swatinemrust-cache@v1.txt" (1865 bytes)
2022-07-31T21:52:45.182151Z  INFO download_gha_logs: File 3 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/5_Run cargo check.txt" (679 bytes)
2022-07-31T21:52:45.183686Z  INFO download_gha_logs: File 4 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/3_Setup rust.txt" (3592 bytes)
2022-07-31T21:52:45.185335Z  INFO download_gha_logs: File 5 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/9_Run cargo-toml-lint.txt" (789 bytes)
2022-07-31T21:52:45.186696Z  INFO download_gha_logs: File 6 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/1_Set up job.txt" (2534 bytes)
2022-07-31T21:52:45.192735Z  INFO download_gha_logs: File 7 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/2_Check out the repository.txt" (9508 bytes)
2022-07-31T21:52:45.195350Z  INFO download_gha_logs: File 8 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/"
2022-07-31T21:52:45.195425Z  INFO download_gha_logs: File 9 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/1_Lint and test.txt" (25192 bytes)
2022-07-31T21:52:45.198829Z  INFO download_gha_logs: File 10 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/19_Post Run Swatinemrust-cache@v1.txt" (96 bytes)
2022-07-31T21:52:45.199506Z  INFO download_gha_logs: File 11 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/6_Run cargo fmt.txt" (398 bytes)
2022-07-31T21:52:45.200096Z  INFO download_gha_logs: File 12 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/22_Complete job.txt" (59 bytes)
2022-07-31T21:52:45.200627Z  INFO download_gha_logs: File 13 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/8_Run cargo test.txt" (1190 bytes)
2022-07-31T21:52:45.201208Z  INFO download_gha_logs: File 14 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/1_Lint and test (1).txt" (456 bytes)
2022-07-31T21:52:45.201826Z  INFO download_gha_logs: File 15 extracted to "./andrewthetechie/download-gha-logs-rs/2766205906/Lint and test/7_Run clippy.txt" (700 bytes)
```

### Download multiple runs

```shell
download-gha-logs andrewthetechie/download-gha-logs-rs --run-ids 2766205906 --run-ids 2766205906`
2022-07-31T21:53:57.366777Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205906
2022-07-31T21:53:57.367941Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205906
2022-07-31T21:53:58.551573Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205906.zip
2022-07-31T21:53:58.551865Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205906.zip
```

### Lookup Run IDs from the Github API

You can use any combination of the options on the CLI to filter looking up runs from the github api.

Count is the number of runs to download, sorted newest to oldest. Offset is the number to skip counting from 0

```shell
download-gha-logs-rs --status 'completed' --branch 'main' --count 5
   Compiling download-gha-logs v0.1.0 (/Users/andrew/Documents/code/download-gha-logs-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 3.74s
     Running `target/debug/download-gha-logs andrewthetechie/download-gha-logs-rs --status completed --branch main --count 5`
2022-07-31T21:57:47.452111Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205903
2022-07-31T21:57:47.452424Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205906
2022-07-31T21:57:47.452541Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766205905
2022-07-31T21:57:47.452656Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766176241
2022-07-31T21:57:47.452759Z  INFO download_gha_logs: Downloading logs for andrewthetechie/download-gha-logs-rs run 2766176240
2022-07-31T21:57:50.245188Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205903.zip
2022-07-31T21:57:50.245308Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205906.zip
2022-07-31T21:57:50.245330Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766205905.zip
2022-07-31T21:57:50.245442Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766176241.zip
2022-07-31T21:57:50.245501Z  INFO download_gha_logs: Writing logs to ./andrewthetechie/download-gha-logs-rs/2766176240.zip
```

## Development

The project supports the use of [tokio-rs-console] for debugging/tracing `async` scenarios. Just
enable the `tokio-console` feature.

[tokio-rs-console]: https://github.com/tokio-rs/console
