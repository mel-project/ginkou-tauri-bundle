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

Then bundle the project
```bash
tauri build
```
