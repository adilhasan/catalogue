// Library of functions for scanning the database
//
use std::{path::PathBuf, fs};
use std::path::Path;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read};
use std::time::{UNIX_EPOCH};
use walkdir::{WalkDir};
use log::{debug, error};
use crate::catalogue_database;
use crate::models;

// Starting from the top_dir walk through the directory finding files of interest, extract properties
// and output a list of files
fn find_files(top_dir: PathBuf, recursive: bool) -> Result<Vec<models::DataFile>, Box <dyn Error>> {
    let mut files: Vec<models::DataFile> = Vec::new();
    let formats = vec!["docx", "pdf", "epub"];

    let mut walk = WalkDir::new(&top_dir).max_depth(1);
    
    if recursive {
        walk = WalkDir::new(&top_dir);
    }
    // loop over the folder and get the 
    for node in walk {
        let path = match node {
            Ok(n) => n.into_path(),
            Err(err) => {
                let e_path = err.path().unwrap_or(Path::new("")).to_path_buf();
                error!("Cannot access path {:?}", e_path);
                e_path
            },
        };

        let mut a_file : models::DataFile = models::DataFile::new();
        a_file.extension = match &path.extension() {
            Some(p_ext) => p_ext.to_str().unwrap().to_string(),
            None => String::from(""),
        };

        if !(formats.iter().any(|&f| f == a_file.extension)) {
            continue;
        }

        let metadata = fs::metadata(&path)?;

        a_file.created = match metadata.created() {
            Ok(t) => match t.duration_since(UNIX_EPOCH) {
                    Ok(ts) => ts.as_secs(),
                    Err(_) => 0,
            },
            Err(_) => 0,
        };

        a_file.size = metadata.len();
        a_file.path = path;
        let input = File::open(&a_file.path)?;
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader)?;
        a_file.hash = HEXUPPER.encode(digest.as_ref());

        debug!("{:?}",&a_file);

        files.push(a_file)

    }
    Ok(files)
}

// Code taken from the Rust Cookbook
// Compute the sha-256 hash for the file
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box <dyn Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

// Scan the directory, get the list of files and put into the database
pub fn scan(directory: PathBuf, recursive: bool, config: models::Config) -> Result<(), Box <dyn Error>> {
    let files = find_files(directory, recursive)?;
    catalogue_database::create_table(&config)?;
    catalogue_database::insert_files(&config, files);
    Ok(())
}