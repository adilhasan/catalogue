// This library module contains all the modules and functions that we will use
mod scan;
mod annotate;
mod catalogue_database;
mod models;

pub use scan::scan;
pub use annotate::annotate;
pub use annotate::list_records;
pub use models::Config;