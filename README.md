# DEMSF-RS - Download and extract Microsoft Sharepoint file crafted using Rust

I've created this `Rust` utility program designed for SREs, DevOps, and Developers to download files from a Microsoft Sharepoint repository.  
This project has undergone a rewrite from its original source at <https://github.com/phlbrz/demsf>, which originally utilized `bash` utility scripts. The process of transitioning to Rust was initiated with the assistance of [ChatGPT](https://chat.openai.com/).  
These scripts are primarily intended for automation in CI pipelines or local setups (developer environment).  
As the Sharepoint link I share is public and doesn't require authentication, the utility provides seamless access.  
The current implementation utilizes some [crates](https://crates.io/) for downloading and Unzip for extraction, but if you prefer an alternative extractor, please don't hesitate to let me know by raising an issue.

[![Build status](https://github.com/rust-lang/regex/workflows/ci/badge.svg)](https://github.com/phlbrz/demsf-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/regex.svg)](https://crates.io/crates/demsf-rs)

## Repository structure

```bash
tree -I target demsf-rs/

demsf-rs/
├── Cargo.lock
├── Cargo.toml # libs from crates.io
├── LICENSE
├── README.md
└── src
    ├── args.rs # a struct and impl mod to encapsulate args from input.
    ├── download.rs # Download a Microsoft Sharepoint File.
    ├── lib.rs # Declared mods.
    ├── main.rs # Execute the program.
    └── unzip.rs # Unzip a file downloaded from Microsoft Sharepoint.
```

## Requirements

- Linux
- Rust

## Usage

- Create your workspace

```bash
mkdir $HOME/workspace-rust/
cd $HOME/workspace-rust/
```

- Clone this repository

```bash
git clone https://github.com/phlbrz/demsf-rs.git
cd demsf-rs

OUTPUT_FOLDER="/home/user/workspace/demsf_output/"
OUPUT_FILENAME="file-name.zip"
URL="https://some-shared-repo.sharepoint.com/:u:/s/SharedRepo/XXXxXXxXxXxXxxXxXxxxxxxXXx1xxxx2X3X4XxxxXXxXXX?e=XXxxX1"

# If you want to extract a zip
cargo run -- "$OUTPUT_FOLDER" "$OUTPUT_FILENAME" "$URL" true

# If you don't need to extract a zip
cargo run -- "$OUTPUT_FOLDER" "$OUTPUT_FILENAME" "$URL" false
```

## How to use the crate

```rust

```

- Rust file `download.rs`: Download a Microsoft Sharepoint File

This script facilitates the retrieval of files from Microsoft Sharepoint.  
It leverages the `args.rs` module, prepopulated with `OUTPUT_FOLDER`, `OUTPUT_FILENAME`, and `URL`.  

- Rust file `unzip.rs`: Unzip a file downloaded from Microsoft Sharepoint

This will unzip the file.  
It leverages the `args.rs` module, prepopulated with `OUTPUT_FOLDER`, `OUTPUT_FILENAME`, `URL` and you **must** set `true` calling the program in command line.

Sources:

- <https://github.com/phlbrz/demsf>
