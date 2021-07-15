// Library of models

use std::{path::PathBuf};
use serde::{Serialize, Deserialize};

// Structure for the database DataFile
#[derive(Debug)]
pub struct DataFile {
    pub created: u64,
    pub path: PathBuf,
    pub size: u64,
    pub title: String,
    pub description: String,
    pub hash: String,
    pub publisher: String,
    pub extension: String,
    pub read: bool,
}

// Annotation to be uploaded to the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub publisher: String,
    pub read: bool,
}

// Annotation record downloaded from the database
#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationRecord {
    pub id: u64,
    pub path: PathBuf,
    pub title: String,
    pub description: String,
    pub publisher: String,
    pub read: bool,
}


// Constructor for the DataFile object
impl DataFile {
    pub fn new() -> Self {
        Self {
            created: 0,
            path: PathBuf::new(),
            size: 0,
            title: String::new(),
            description: String::new(),
            hash: String::new(),
            publisher: String::new(),
            extension: String::new(),
            read: false,
        }
    }
}

// Config parameters
#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
}

// Database table schema
#[derive(Debug, Deserialize)]
pub struct Database {
    pub file: PathBuf,
    pub get_path: String,
    pub create_catalogue: String,
    pub insert_catalogue: String,
    pub update_catalogue: String,
    pub all_annotation: String,
    pub before_annotation: String,
    pub after_annotation: String,
    pub interval_annotation: String,
    pub query_d: String,
    pub query_t: String,
    pub update_record: String,
}
