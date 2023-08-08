/**
 * DEMSF - Rust - Download and extract Microsoft Sharepoint file
 * Rewrite of https://github.com/phlbrz/demsf.
 *
 * Extract zip.
 * Source: https://github.com/zip-rs/zip/blob/master/examples/extract.rs
 */
use log::debug;
use std::fs;
use std::io;
use std::path::Path;
use zip::read::ZipArchive;

use crate::args::Args;

pub fn unzip(args: &Args) {
    // Extract the zip file
    let output_folder = Path::new(&args.output_folder);
    debug!("folder: {}", output_folder.display());

    let zip_file = fs::File::open(&args.full_path_file).expect("Failed to open file.");
    let mut archive = ZipArchive::new(zip_file).expect("Failed to open zip.");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to find index.");
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                debug!("File {i} comment: {comment}");
            }
        }

        let complete_outpath = output_folder.join(outpath);

        if (*file.name()).ends_with('/') {
            debug!("File {} extracted to \"{}\"", i, complete_outpath.display());
            fs::create_dir_all(&complete_outpath).unwrap();
        } else {
            debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                complete_outpath.display(),
                file.size()
            );
            if let Some(p) = complete_outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&complete_outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                debug!(
                    "Set permissions {} to file {}",
                    mode,
                    &complete_outpath.display()
                );
                fs::set_permissions(&complete_outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
