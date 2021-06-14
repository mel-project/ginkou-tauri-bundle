#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::Path;
use tauri::api::process::{Command, CommandChild, CommandEvent};
use tauri::async_runtime::Receiver;

fn start_walletd(wallet_dir: &Path) -> tauri::Result<(Receiver<CommandEvent>, CommandChild)> {
//fn start_walletd(wallet_dir: &Path) {
    let wallet_path_str = wallet_dir.to_str()
        .expect(&format!("Invalid wallet directory path, {}", wallet_dir.to_string_lossy()));

    // Appends the target triplet and determines relative path from the program name
    let proc = Command::new_sidecar("melwalletd")
        .expect("Couldn't find melwalletd binary");

    proc.args(&["--listen", "127.0.0.1:11773"])
        .args(&["--mainnet-connect", "51.83.255.223:11814"])
        .args(&["--testnet-connect", "94.237.109.44:11814"])
        .args(&["--wallet-dir", wallet_path_str])
        .spawn()
        .map_err(|e| tauri::Error::FailedToExecuteApi(e))
}

fn main() {
    // Spawn wallet daemon as a child process
    let wallet_dir = Path::new("./wlt");
    let (_, cmd) = start_walletd(wallet_dir)
        .expect("melwalletd failed to start");

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Kill melwalletd
    cmd.kill()
        .expect("Failed to kill melwalletd");
}
