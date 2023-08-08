/**
 * DEMSF - Rust - Download and extract Microsoft Sharepoint file
 * Rewrite of https://github.com/phlbrz/demsf.
 *
 * Args utility.
 */
use log::{debug, error};
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

pub struct Args {
    pub output_folder: String,   // desired output folder where file will be at.
    pub output_filename: String, // desired output filename.
    pub full_path_file: String,  // full path to the file.
    pub url: String,             // Microsoft Sharepoint url to download the file.
    pub unzip: String,           // true or false to unzip the file.
}

impl Args {
    pub fn parse_arguments() -> Result<Args, &'static str> {
        // https://doc.rust-lang.org/rust-by-example/std_misc/arg.html
        // Read command line arguments
        let args: Vec<String> = env::args().collect();
        debug!("Parse args: {:#?}", &args);
        if args.len() < 5 {
            error!("parse_arguments - Usage: {} <output_folder> <output_filename> <url> <true|false>(unzip)", args[0]);
        }
        Ok(Args {
            output_folder: args[1].to_string(),
            output_filename: args[2].to_string(),
            full_path_file: format!("{}/{}", args[1].to_string(), args[2].to_string()),
            url: args[3].to_string(),
            unzip: args[4].to_string(),
        })
    }

    pub fn replace_query_url(&mut self) {
        // Regex editor and tester: https://rustexp.lpil.uk/
        // Replace '?e=xxx' http query parameter with '?download=1'
        debug!("URL original: {}", &self.url);

        let match_regex = r"\?+(.*)$";
        let replace = "?download=1";
        
        let regex = Regex::new(match_regex).unwrap();
        self.url = regex.replace(&self.url[..], replace).to_string();
        
        debug!("URL replaced: {}", &self.url);
    }

    pub fn create_dir(&self) {
        // Check if the output folder exists, create it if not
        debug!("Create dir: {}", &self.output_folder);
        if !Path::new(&self.output_folder).is_dir() {
            fs::create_dir_all(&self.output_folder).expect("Failed to create the output folder.");
        }
    }
}
