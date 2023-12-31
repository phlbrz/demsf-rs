# DEMSF-RS - Download and extract Microsoft Sharepoint file crafted using Rust

I've created this `Rust` utility program designed for SREs, DevOps, and Developers to download files from a Microsoft Sharepoint repository.  
This project has undergone a rewrite from its original source at <https://github.com/phlbrz/demsf>, which originally utilized `bash` utility scripts. The process of transitioning to Rust was initiated with the assistance of [ChatGPT](https://chat.openai.com/).  
These scripts are primarily intended for automation in CI pipelines or local setups (developer environment).  
As the Sharepoint link I share is public and doesn't require authentication, the utility provides seamless access.  
The current implementation utilizes some [crates](https://crates.io/) for downloading and Unzip for extraction, but if you prefer an alternative extractor, please don't hesitate to let me know by raising an issue.

[![Build status](https://github.com/phlbrz/demsf-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/phlbrz/demsf-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/demsf-rs)](https://crates.io/crates/demsf-rs)

## Repository structure

```bash
tree -I target demsf-rs/

demsf-rs/
├── Cargo.toml # crate/libs from crates.io
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

- To download a file from Microsoft Sharepoint, unzip **must** set to `false`.
- To unzip a file from filesystem, unzip **must** set to `true`, or don't call download function.

```rust
use log::{debug, error};

use demsf_rs::args::Args;
use demsf_rs::download::download;
use demsf_rs::unzip::unzip;

fn main() {
    env_logger::init();
    match Args::parse_arguments() {
        Err(why) => {
            error!("Error");
            panic!("{:?}", why)
        }
        Ok(mut value) => {
            debug!("call download."); // if you don't want to download, comment this line.
            download(&mut value); // if you don't want to download, comment this line.
            debug!("call unzip? value={:#?}", &value.unzip); // if you want to unzip, set true
            match &value.unzip.parse() {
                Ok(true) => unzip(&value),
                Ok(false) => (),
                Err(_) => panic!("Only true or false is valid for unzip option."),
            }
        }
    }
}
```

- Rust file `download.rs`: Download a Microsoft Sharepoint File

This script facilitates the retrieval of files from Microsoft Sharepoint.  
It leverages the `args.rs` module, prepopulated with `OUTPUT_FOLDER`, `OUTPUT_FILENAME`, and `URL`.  

- Rust file `unzip.rs`: Unzip a file downloaded from Microsoft Sharepoint

This will unzip the file.  
It leverages the `args.rs` module, prepopulated with `OUTPUT_FOLDER`, `OUTPUT_FILENAME`, `URL` and you **must** set `true` calling the program in command line.

Sources:

- <https://github.com/phlbrz/demsf>
