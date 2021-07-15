use structopt::StructOpt;
use std::path::PathBuf;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use env_logger::Builder;
// use log::{debug, error, log_enabled, info, trace, warn};
use category::{scan, annotate, list_records};
use category::Config;

// These are our command line options
#[derive(StructOpt, Debug)]
#[structopt(name = "categorise", about = "Categorise the documents in a directory")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short, long)]
    config: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Scan the specified directory
    /// 
    /// This command scans the directory searching for
    /// documents and then saves the metadata to a database
    Scan { 
        directory: PathBuf,

        #[structopt(short, long)]
        recursive: bool,
    },
    /// Annotate the database entries
    /// 
    /// This command allows users to annotate database records
    /// by allowing the user to upload a JSON document of records
    Annotate {
        upload_file: PathBuf,
    },

    /// List of database records
    /// 
    /// This command returns a JSON file of database records 
    List {
        output_file: PathBuf,
    
        #[structopt(short = "a", long = "after")]
        after_date: Option<String>,
    
        #[structopt(short = "b", long = "before")]
        before_date: Option<String>,

        #[structopt(short = "s", long = "search")]
        search_string: Option<String>,

        #[structopt(short = "d", long = "description", help = "description is empty")]
        description: bool,

        #[structopt(short = "t", long = "title", help = "title is empty")]
        title: bool,
    },
}


fn get_default_path(file_name: String) -> PathBuf {
    // Get the current path - we expect the TOML config file to be here
    let mut db_path = env::current_dir().expect("expected current directory");
    db_path.push(file_name);
    db_path
}

// Function to read in the TOML file and output a configuration structure
fn read_config(config_file: PathBuf) -> Config {
    // Read in the config file, extract the parameters and return the config object
    let mut a_file = File::open(config_file).expect("not found");
    let mut contents = String::new();

    a_file.read_to_string(&mut contents).expect("could not read file");
    // Use serde toml to deserialize
    let config: Config = toml::from_str(&contents).expect("could not unpack toml");
    config
}

fn main() -> Result<(), Box<dyn Error>> {

    //Initiate the logger
    let mut builder = Builder::from_env("LOGLEVEL");
    builder.init();

    // Declare the command line options and arguments
    let opt = Opt::from_args();

    // Get the config TOML file
    let config_file: PathBuf = match opt.config {
        Some(path) => path,
        None => get_default_path("config.toml".to_string()),
    };

    let config = read_config(config_file);

    // Match the supplied sub-command to the correct function
    match opt.cmd {
        Command::Scan {directory, recursive} => scan(directory, recursive, config),
        Command::List {output_file, after_date, before_date, search_string, description, title} => list_records(output_file, after_date, before_date, search_string, description, title, config),
        Command::Annotate {upload_file} => annotate(upload_file, config),
    }

}
