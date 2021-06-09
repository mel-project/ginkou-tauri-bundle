#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
    path::Path,
    process::Command,
};

fn start_walletd(wallet_dir: &Path) {

    let wallet_path_str = wallet_dir.to_str()
        .expect(&format!("Invalid wallet directory path, {}", wallet_dir.to_string_lossy()));

    Command::new("./bin/melwalletd")
        .args(&["--listen", "127.0.0.1:11773"])
        .args(&["--mainnet-connect", "51.83.255.223:11814"])
        .args(&["--testnet-connect", "94.237.109.44:11814"])
        .args(&["--wallet-dir", wallet_path_str])
        .spawn()
        .expect("melwalletd failed to start");
}

fn main() {
    // Spawn wallet daemon as a child process
    let wallet_dir = Path::new("./wlt");
    start_walletd(wallet_dir);

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
