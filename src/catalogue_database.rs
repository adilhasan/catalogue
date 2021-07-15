// Library of database functions
//
extern crate serde;
use std::path::{Path, PathBuf};
use crate::models;

// Create a database table
pub fn create_table(config: &models::Config) -> rusqlite::Result<()> {
    
    let conn = rusqlite::Connection::open(config.database.file.as_path())?;
    
    conn.execute(&config.database.create_catalogue, [],)?;
    Ok(())
}

// Insert a list of files into a database
pub fn insert_files(config: &models::Config, files: Vec<models::DataFile>) {
    // Loop over the vector and then check if each file exists in the database based on name.
    // if it does I should check the hash. If it's not the same just print out that there is 
    // a difference for now.
    let conn =  rusqlite::Connection::open(config.database.file.as_path()).unwrap();
    for a_file in files {
        let cand_path: String = match conn.query_row(&config.database.get_path,
        rusqlite::params![a_file.hash],|row| row.get(0)) {
            Ok(p) => p,
            Err(_) => String::from(""),
        };

        if cand_path.is_empty() {
            let a_path = match a_file.path.into_os_string().into_string() {
                Ok(p) => p,
                Err(_) => String::from(""),
            };
            match conn.execute(&config.database.insert_catalogue,
                                rusqlite::params![a_file.size, a_path, a_file.hash, a_file.extension,
                                                    a_file.created, a_file.read]) {
                                                        Ok(added) => println!("{} row added", added),
                                                        Err(err) => println!("Addition failed {}", err),
            };
        } else if Path::new(&cand_path) != a_file.path {
            if Path::new(&cand_path).exists() {
                    println!("Duplicate file original: {} new: {:?}", cand_path, a_file.path);
            } else {
                let a_path = match a_file.path.into_os_string().into_string() {
                    Ok(p) => p,
                    Err(_) => String::from(""),
                };

                match conn.execute(&config.database.update_catalogue,
                                    rusqlite::params!(a_path, a_file.hash)) {
                                        Ok(updated) => println!("{} row updated", updated),
                                        Err(err) => println!("Update failed {}", err)
                                    };
                }
        }       
    }
}

// Update the database with the contents of the record
pub fn update_record(record: models::Annotation, config: &models::Config) {
    let conn =  rusqlite::Connection::open(config.database.file.as_path()).unwrap();
    match conn.execute(&config.database.update_record,
        rusqlite::params![record.title, record.description, record.publisher,
                            record.read, record.id]) {
                                Ok(added) => println!("{} row updated", added),
                                Err(err) => println!("Update failed {}", err),
                            };
}

// Get a list of records from the database
pub fn list_records(a_date: i64, b_date: i64, search: String, description: bool, title: bool, 
    config: &models::Config) -> rusqlite::Result<Vec<models::AnnotationRecord>> {
    // Create a connection and get all the database entries
    let mut sole = false;
    let records: Vec<models::AnnotationRecord>;
    let conn =  rusqlite::Connection::open(&config.database.file.as_path()).unwrap();

    if a_date > 0 && b_date > 0 {
        let query = assemble_query(&config.database.interval_annotation, 
            description, title, sole, config);
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query([a_date, b_date])?;
        records = get_records(rows, search);
    } else if a_date > 0 {
        let query = assemble_query(&config.database.after_annotation, 
            description, title, sole, config);
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query([a_date])?;
        records = get_records(rows, search);
    } else if b_date > 0 {
        let query = assemble_query(&config.database.before_annotation, 
            description, title, sole, config);
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query([b_date])?;
        records = get_records(rows, search);
    } else {
        sole = true;
        let query = assemble_query(&config.database.all_annotation, 
            description, title, sole, config);
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query([])?;
        records = get_records(rows, search);
    }
    Ok(records)
}

// Create the SQL query
fn assemble_query(query: &str, description: bool,
    title: bool, sole: bool, config:&models::Config) -> String {

        let mut out_query = query.to_owned();
        if description {
            if sole {
                out_query = format!("{} where {}", &out_query, &config.database.query_d);
            } else {
                out_query = format!("{} and {}", &out_query, &config.database.query_d);
            }
        }
        if title {
            if sole && !description {
                out_query = format!("{} where {}", &out_query, &config.database.query_t);
            } else {
                out_query = format!("{} and {}", &out_query, &config.database.query_t);
            }
        }
        out_query
}

// Get the list of records from the database
fn get_records(mut rows:rusqlite::Rows, search: String) -> Vec<models::AnnotationRecord> {
    let mut records: Vec<models::AnnotationRecord> = Vec::new();
    while let Some(row) = rows.next().expect("") {
        let record = models::AnnotationRecord{
            id : row.get(0).expect(""),
            path : PathBuf::from(row.get::<usize, String>(1).expect("").to_string()),
            title : match row.get::<usize, String>(2) {
                Ok(t) => t.to_string(),
                Err(_) => "".to_string(),
            },
            description : match row.get::<usize, String>(3) {
                Ok(d) => d.to_string(),
                Err(_) => "".to_string(),
            },
            publisher : match row.get::<usize, String>(4) {
                Ok(p) => p.to_string(),
                Err(_) => "".to_string(),
            },
            read : match row.get::<usize, bool>(5) {
                Ok(r) => r,
                Err(_) => false,
            },
        };
        if search.len() == 0 {
            records.push(record);
        } else if record.title.contains(&search) || record.description.contains(&search) {
            records.push(record);
        }
    }
    records
}