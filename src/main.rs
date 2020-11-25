use std::path::{Path, PathBuf};

use log::{info, error};
use tokio::io::AsyncWriteExt;

const VSCODE_DOWNLOAD_URL: &str = "https://update.code.visualstudio.com/latest/linux-deb-x64/stable";

fn main() {
    log_init();

    if let Err(err) = run() {
        error!("{}", err);
        std::process::exit(1);
    }
}

fn log_init() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Tokio 2
    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .max_threads(4)
        .build()?;

    runtime.block_on(main_task())?;

    Ok(())
}

async fn main_task() -> Result<(), Box<dyn std::error::Error>> {
    info!("downloading the latest vscode...");
    let output_file_path: PathBuf = download_vscode().await?;

    info!("installing vscode...");
    install_vscode(&output_file_path).await?;

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

    // Try to get the filename of the last url segment. If it fails, use "vscode.deb" as default
    let file_name: &str = response.url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("vscode.deb");

    let mut output_file_path = std::env::temp_dir();
    output_file_path.push(file_name);

    // Creates a new file and asynchronously writes buffers to it that are comming from the response stream
    let mut output_file: tokio::fs::File = tokio::fs::File::create(&output_file_path).await?;
    while let Some(chunk) = response.chunk().await? {
        output_file.write(&chunk).await?;
    }

    Ok(output_file_path)
}

async fn install_vscode<P>(output_file_path: P) -> Result<(), Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let output_file_path: &str = output_file_path.as_ref()
        .to_str()
        .ok_or("output path is not a valid utf-8 string")?;

    let result = std::process::Command::new("sudo")
        .args(&["dpkg", "-i", output_file_path])
        .status()?;
    if !result.success() {
        return Err("dpkg failed".into());
    }

    Ok(())
}
