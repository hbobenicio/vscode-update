mod logging;

use std::path::{Path, PathBuf};

use log::{info, error};
use std::io::{Read, Write};

const VSCODE_DOWNLOAD_URL: &str = "https://update.code.visualstudio.com/latest/linux-deb-x64/stable";

fn main() {
    logging::init();

    if let Err(err) = run() {
        error!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    main_task()?;
    Ok(())
}

fn main_task() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut data: Vec<u8> = Vec::new();
    response.read_to_end(&mut data)?;

    let output_file_path = vscode_file_path();
    let mut output_file: std::fs::File = std::fs::File::create(&output_file_path)?;

    output_file.write_all(&data)?;

    Ok(output_file_path)
}

fn vscode_file_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("vscode.deb");

    path
}

fn install_vscode<P>(output_file_path: P) -> Result<(), Box<dyn std::error::Error>>
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
