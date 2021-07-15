// Library to annotate the database
extern crate chrono;

use std::error::Error;
use std::path::PathBuf;
use crate::catalogue_database;
use crate::models;
use chrono::prelude::*;

// Annotate the database by taking as input the records in the JSON to annotate and the config file
pub fn annotate(upload_file: PathBuf, config: models::Config) -> Result<(), Box <dyn Error>> {

    let file_in = std::fs::File::open(upload_file).expect("unable to open file");
    // Used serde to parse the contents of the JSON file
    let records: Vec<models::Annotation> = serde_json::from_reader(file_in).expect("cannot parse json");
    // Loop over the list of records and update the database
    for a_record in records {
        catalogue_database::update_record(a_record, &config);
    }
    Ok(())
}

// Get a list of records from the database and write the list to a JSON file
pub fn list_records(output_file: PathBuf, after_date: Option<String>, before_date: Option<String>, 
    search_string: Option<String>, description: bool, title: bool, config: models::Config) -> Result<(), Box<dyn Error>> {
    
    // Check the after date is in the correct format and store as a timestamp
    let a_date = match after_date {
        Some(a) => Utc.datetime_from_str(&a, "%Y-%m-%d %H:%M:%S").expect("Unable to parse date").timestamp(),
        None => 0,
    };

    // Check the before date is in the correct format and store as a timestamp
    let b_date = 
    match before_date {
        Some(b) => Utc.datetime_from_str(&b, "%Y-%m-%d").expect("Unable to parse date").timestamp(),
        None => 0,
    };

    // Get the search string
    let search = match search_string {
        Some(s) => s,
        None => "".to_string(),
    };

    
    // Read in the database and then get the list of records from the database
    let records = match catalogue_database::list_records(a_date, b_date, search, description, title, &config) {
        Ok(p) => p,
        Err(_) => Vec::new(),
    };

    // Output the list of records as JSON objects to a file
    let file_out = std::fs::File::create(output_file).expect("file create failed");
    serde_json::to_writer_pretty(&file_out, &records).expect("Cannot write to file");
    Ok(())
}