use chrono::prelude::*;
use std::fs::remove_file;
use std::path::Path;
use std::process;

mod db;
mod records;
mod utils;

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
    let mut db = db::new(FILE_PATH_DB);
    let mut num_rows_affected: usize;
    let records = preprocessor.read_data().unwrap();
    db.create_table().unwrap();
    num_rows_affected = db.populate_table_tx(&records).unwrap();

    if let Some(birthdays) = db.fetch_all() {
        if birthdays.len() > 0 {
            // Get friends having birthdays today
            let dt = chrono::Local::now();
            let month_today = dt.month();
            let day_today = dt.day();
            let people: Vec<utils::Person> =
                utils::happy_birthday(month_today, day_today, &birthdays);
            println!(
                "==> There are {} people having their birthday today!",
                people.len()
            );
            for person in people {
                println!("{} {}", person.firstname, person.lastname);
            }
        } else {
            println!("Nothing for today!");
            process::exit(0);
        }
    } else {
        println!("Nothing for today!");
        process::exit(0);
    }
}
