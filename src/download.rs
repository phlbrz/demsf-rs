/**
 * DEMSF - Rust - Download and extract Microsoft Sharepoint file
 * Rewrite of https://github.com/phlbrz/demsf.
 *
 * Download.
 *
 */
use log::{debug, error};
use reqwest::blocking::{Client, Response};
use std::fs::File;
use std::io::prelude::*;

use crate::args::Args;

pub fn download(args: &mut Args) {
    args.replace_query_url();
    args.create_dir();
    write(&args, request(&args));
}

fn request(args: &Args) -> Result<Response, Box<dyn std::error::Error>> {
    debug!("Downloading from {}", &args.url);
    let client = Client::builder().cookie_store(true).build()?;

    let response = client.get(&args.url).send()?;

    // Check if the request was successful
    if !response.status().is_success() {
        error!("Failed to download the file. Status code: {}", response.status());
        return Err(format!("Failed to download the file. Status code: {}", response.status()).into());
    }
    debug!("Download successful from {}", &args.url);
    Ok(response)
}

fn write(args: &Args, response: Result<Response, Box<dyn std::error::Error>>) {
    // Write the response content to the output file
    match response {
        Ok(value) => {
            debug!("Writing file {}", &args.full_path_file);
            let mut output_file = File::create(&args.full_path_file).expect("Failed to create file.");
            let bytes = value.bytes().expect("Failed to retrive file.");
            output_file.write_all(&bytes).expect("Failed to write file.");
            debug!("Download and write successful of file {}",&args.full_path_file);
        }
        Err(e) => error!("Error downloading the file {}", e),
    }
}
