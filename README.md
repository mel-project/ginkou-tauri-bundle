# Ginkou Bundler
This project provides the means to bundle the Ginkou wallet javascript UI into a
native binary application.

Tauri is a means of porting a web view to native (like Electron but
light-weight and written in Rust).

## Usage
To bundle an application, you must either install and configure dependencies manually,
or use nix package manager. With nix installed, in the top-level directory of
this repo, run this to open a shell:

```bash
nix develop
```

*This step will be removed once ginkou has its own nix flake*
Build the wallet UI
```bash
cd ginkou
npm install
npm run build
```

Then bundle the project
```bash
# At the root of this repository
tauri build
sh build_tar.tar.gz
```

There will be a ginkou.tar.gz file in src-tauri/target/release.

If you happen to get an `Permission denied (os error 13)` when building, try
running `cargo clean`.

## What is the nix shell configuring?
This project expects the entire ginkou project as a directory in the top level
of this repository. Instead of using a (slightly confusing) submodule, the nix flake
downloads ginkou and places it at the top level. The flake.lock file tracks the
version of ginkou to fetch.

The flake shell also downloads and builds melwalletd for your system, and
places the binary `melwalletd-<target-triplet>` in the src-tauri directory.

The nix flake also specifies all dependencies needed for building these
projects and bundling with tauri. This includes gtk3 which, even if you have on
your system, will be rebuilt, which can take hours. This will only happen the
first time you run `nix develop`. Making gtk3 optional for impure builds will
be a future improvement.
