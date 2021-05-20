use std::fs::remove_file;
use std::path::Path;

mod db;
mod records;

const FILE_PATH_CSV: &str = "./data/bdays.csv";
const FILE_PATH_DB: &str = "./data/bdays.db";

const RESET: bool = true;

fn main() {
    if RESET {
        if Path::new(FILE_PATH_DB).exists() {
            remove_file(FILE_PATH_DB).unwrap();
        }
    }
    let preprocessor = records::RecordPreprocessor {
        file_path: FILE_PATH_CSV.to_string(),
    };
    let db = db::new(FILE_PATH_DB);
    let records = preprocessor.read_data().unwrap();
    db.create_table().unwrap();
    db.populate_table(&records);
}
