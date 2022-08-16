#![doc = include_str!("../README.md")]

use anyhow::Result;
use octocrab::Octocrab;

use self::args::Args;
use bytes::Bytes;
use clap::Parser;
use futures::future::join_all;
use octocrab::Error as OctocrabError;
use std::fs;
use std::io;
use tracing::error as error_log;
use tracing::info as info_log;

mod args;
mod log;

async fn extract_file(path_str: String) -> Result<()> {
    let fname = std::path::Path::new(&*path_str);
    let parent = fname.parent().unwrap();
    let parent_path = parent.join(fname.file_stem().unwrap());
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let fpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let outpath = parent_path.join(fpath);
        {
            let comment = file.comment();
            if !comment.is_empty() {
                info_log!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            info_log!("File {} extracted to \"{}\"", i, outpath.display());
            create_dir_if_not_exist(outpath.to_str().unwrap().to_string())
                .await
                .unwrap();
        } else {
            info_log!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}

async fn create_dir_if_not_exist(path: String) -> Result<(), String> {
    let this_path = std::path::Path::new(&path);
    if !this_path.is_dir() {
        match tokio::fs::create_dir_all(path).await {
            Ok(_) => (),
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    Ok(())
}

async fn write_logs(logs: Bytes, path_str: String, filename_str: String) -> Result<String, String> {
    let fullpath_str = format!("{}/{}", path_str, filename_str);
    info_log!("Writing logs to {}", fullpath_str);
    match create_dir_if_not_exist(path_str.to_string()).await {
        Ok(_) => {
            let path = std::path::Path::new(&fullpath_str);
            match tokio::fs::write(path, logs).await {
                Ok(_) => Ok(fullpath_str),
                Err(e) => {
                    error_log!("Error writing log file to {}: {} ", fullpath_str, e);
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            error_log!("Error creating directory: {}", e);
            Err(format!("Error creating directory: {}", e))
        }
    }
}

/// Download the logs of a Github Action workflow run from repo.
async fn download_logs(
    octocrab: Octocrab,
    repo: String,
    run_id: u64,
) -> Result<(u64, Bytes), String> {
    info_log!("Downloading logs for {} run {}", repo, run_id);
    let split = repo.split('/');
    let repo_split = split.collect::<Vec<&str>>();
    match octocrab
        .actions()
        .download_workflow_run_logs(repo_split[0], repo_split[1], run_id.into())
        .await
    {
        Ok(logs) => {
            let b = std::str::from_utf8(&logs);
            if b.is_ok() && b.unwrap().contains("\"message\":\"Not Found\"") {
                return Err(format!("Run {} not found", run_id));
            }
            Ok((run_id, logs))
        }
        Err(e) => Err(e.to_string()),
    }
}

struct RunFilter {
    workflow_file: Option<String>,
    actor: Option<String>,
    branch: Option<String>,
    event: Option<String>,
    status: Option<String>,
}

async fn lookup_run_ids(
    octocrab: Octocrab,
    repo: String,
    filter: RunFilter,
    count: u32,
    offset: u32,
) -> Result<Vec<u64>, OctocrabError> {
    let split = repo.split('/');
    let repo_split = split.collect::<Vec<&str>>();
    let workflows_handler = octocrab.workflows(repo_split[0], repo_split[1]);
    let mut runs_handler = workflows_handler.list_all_runs();

    if filter.workflow_file != None {
        runs_handler = workflows_handler.list_runs(filter.workflow_file.unwrap());
    }
    if filter.actor != None {
        runs_handler = runs_handler.actor(filter.actor.unwrap());
    }
    if filter.branch != None {
        runs_handler = runs_handler.branch(filter.branch.unwrap());
    }
    if filter.event != None {
        runs_handler = runs_handler.event(filter.event.unwrap());
    }
    if filter.status != None {
        runs_handler = runs_handler.status(filter.status.unwrap());
    }
    runs_handler = runs_handler.per_page(count as u8);
    if offset > 0 {
        let page = ((offset / count) + 1) as u8;
        println!("Page: {}", page);
        runs_handler = runs_handler.page(page);
    }
    match runs_handler.send().await {
        Ok(runs) => Ok(runs.items.iter().map(|r| r.id.0).collect()),
        Err(e) => Err(e),
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    log::setup_tracing();
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let mut run_ids = args.run_ids;
    if run_ids.is_empty() {
        let filter = RunFilter {
            workflow_file: args.workflow_file,
            actor: args.actor,
            branch: args.branch,
            event: args.event,
            status: args.status,
        };
        run_ids = lookup_run_ids(
            octocrab.clone(),
            args.repo.to_string(),
            filter,
            args.count,
            args.offset,
        )
        .await?;
    }

    let mut log_futs = Vec::new();
    for run_id in run_ids {
        let log_fut = download_logs(octocrab.clone(), args.repo.to_string(), run_id);
        log_futs.push(log_fut);
    }
    let logs = join_all(log_futs).await;

    let mut write_futs = Vec::new();
    for log in logs {
        match log {
            Ok((run_id, logs)) => {
                let write_fut = write_logs(
                    logs,
                    format!("{}/{}", args.output_path, args.repo),
                    format!("{}.zip", run_id),
                );
                write_futs.push(write_fut);
            }
            Err(e) => {
                error_log!("Error downloading logs {:?}", e);
            }
        }
    }
    if write_futs.is_empty() {
        info_log!("No workflow runs found.")
    }
    let files = join_all(write_futs).await;

    if args.extract {
        let mut extract_futs = Vec::new();
        for file in files {
            match file {
                Ok(file) => {
                    let extract_fut = extract_file(file);
                    extract_futs.push(extract_fut);
                }
                Err(e) => {
                    error_log!("Error when writing logs. Won't extract {}", e);
                }
            }
        }
        join_all(extract_futs).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::test;

    #[test]
    async fn dummy() -> Result<()> {
        log::setup_tracing();
        tracing::warn!("No actual unit tests yet");
        assert_eq!(4, 3 + 1);
        Ok(())
    }
}
