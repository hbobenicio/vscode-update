use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
// use tokio::prelude::*;

const VSCODE_DOWNLOAD_URL: &str = "https://update.code.visualstudio.com/latest/linux-deb-x64/stable";

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Tokio 2
    let mut runtime = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()?;

    runtime.block_on(main_task())?;

    Ok(())
}

async fn main_task() -> Result<(), Box<dyn std::error::Error>> {
    let output_file_path: PathBuf = download_vscode().await?;

    let result = std::process::Command::new("sudo")
        .args(&["dpkg", "-i", output_file_path.to_str().unwrap()])
        .status()?;
    if !result.success() {
        return Err("dpkg failed".into());
    }

    Ok(())
}

async fn download_vscode() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(10))
        .connection_verbose(true)
        .build()?;

    let mut response: reqwest::Response = client.get(VSCODE_DOWNLOAD_URL)
        .send()
        .await?
        .error_for_status()?; // generate an error if server didn't respond OK

    let file_name: &str = response.url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("vscode.deb");

    let mut output_file_path = std::env::temp_dir();
    output_file_path.push(file_name);

    let mut output_file: tokio::fs::File = tokio::fs::File::create(&output_file_path).await?;
    while let Some(chunk) = response.chunk().await? {
        output_file.write(&chunk).await?;
    }

    Ok(output_file_path)
}

async fn install_vs_code() -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!();
}