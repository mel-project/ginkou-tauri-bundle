#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::api::process::Command;

use std::{
    path::Path,
    //process::{Command, Stdio},
};

//fn start_walletd(wallet_dir: &Path) -> Result<(Receiver<CommandEvent>, CommandChild)> {
fn start_walletd(wallet_dir: &Path) {
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
        .expect("melwalletd failed to start");

    /*
    let triplet_name = command::binary_command("melwalletd")
        .expect("Failed to get architecture triplet name for melwalletd");

    command::spawn_relative_command(
        triplet_name,
        vec![
             "--listen", "127.0.0.1:11773",
             "--mainnet-connect", "51.83.255.223:11814",
             "--testnet-connect", "94.237.109.44:11814",
             "--wallet-dir", wallet_path_str
        ],
        Stdio::piped(),
        )
        .spawn()
        .expect("melwalletd failed to start");
    */
}

fn main() {
    // Spawn wallet daemon as a child process
    let wallet_dir = Path::new("./wlt");
    start_walletd(wallet_dir);

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
