/**
 * DEMSF - Rust - Download and extract Microsoft Sharepoint file
 * Rewrite of https://github.com/phlbrz/demsf.
 *
 * Download and extract (if you wish).
 *
 */
use log::{debug, error};
extern crate demsf_rs;

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
            debug!("call download.");
            download(&mut value);
            debug!("call unzip? value={:#?}", &value.unzip);
            match &value.unzip.parse() {
                Ok(true) => unzip(&value),
                Ok(false) => (),
                Err(_) => panic!("Only true or false is valid for unzip option."),
            }
        }
    }
}
