{ stdenv, system, tauri, melwalletd }:

stdenv.mkDerivation {
  name = "ginkou";
  system = system;

  buildCommand = ''
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"

    # melwalletd cli
    export PATH=$PATH:${melwalletd}/bin
    # tauri cli
    export PATH=$PATH:${packages.tauri}/bin
    alias tauri='cargo-tauri'

    # Bundle project
    tauri build

    # Put the binaries in the derivation location
    cp ./src-tauri/target/release/bundle/ginkou $out/bin/
    cp ${melwalletd}/bin/melwalletd $out/bin
  '';
};
