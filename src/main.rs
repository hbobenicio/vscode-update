mod logging;

use std::fs::File;
use std::path::{Path, PathBuf};

use log::{info, error};

const VSCODE_DOWNLOAD_URL: &str = "https://update.code.visualstudio.com/latest/linux-deb-x64/stable";

fn main() {
    logging::init();

    if let Err(err) = run() {
        error!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("downloading the latest vscode...");
    let output_file_path: PathBuf = download_vscode()?;

    info!("installing vscode...");
    install_vscode(&output_file_path)?;

    Ok(())
}

fn download_vscode() -> Result<PathBuf, Box<dyn std::error::Error>> {

    let mut response: reqwest::blocking::Response = {
        let client: reqwest::blocking::Client = reqwest::blocking::ClientBuilder::new()
            .connection_verbose(true)
            .build()?;

        client.get(VSCODE_DOWNLOAD_URL)
            .send()?
    };

    let tmp_file_path: PathBuf = vscode_tmp_file_path();
    let mut output_file: File = File::create(&tmp_file_path)?;

    std::io::copy(&mut response, &mut output_file)?;

    Ok(tmp_file_path)
}

fn vscode_tmp_file_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("vscode.deb");
    path
}

fn install_vscode<P>(output_file_path: P) -> Result<(), Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let output_file_path: &str = output_file_path.as_ref()
        .to_str()
        .ok_or("vscode tmp download path is not a valid utf-8 string somehow... aborting")?;

    let result = std::process::Command::new("sudo")
        .args(&["dpkg", "-i", output_file_path])
        .status()?;
    if !result.success() {
        return Err("dpkg failed".into());
    }

    Ok(())
}
