#[macro_use]
extern crate clap;

use clap::{App, Arg};

use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
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
    // Command-line argument parsing
    let matches = App::new("rsbday")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Check out the people you love who are having birthdays recently!")
        .get_matches();

    if RESET {
        if Path::new(FILE_PATH_DB).exists() {
            remove_file(FILE_PATH_DB).unwrap();
        }
    }

    let mut db = db::new(FILE_PATH_DB);

    if RESET {
        let preprocessor = records::RecordPreprocessor {
            file_path: FILE_PATH_CSV.to_string(),
        };
        let records = preprocessor.read_data().unwrap();
        db.create_table().unwrap();
        db.populate_table_tx(&records).unwrap();
    }

    if let Some(birthdays) = db.fetch_all() {
        if birthdays.len() > 0 {
            let dt = chrono::Local::today();
            println!("Today is {}.\n", dt.format("%a %b %e %Y").to_string());

            // Get people having birthdays between today - 7 days and today + 7 days.
            let mut map_dates: HashMap<String, Date<Local>> = HashMap::new();
            for offset in -7..7 {
                let dt = dt + Duration::days(offset);
                let month = dt.month();
                let day = dt.day();

                let mut str_month: String = month.to_string();
                let mut str_day: String = day.to_string();

                if str_month.len() < 2 {
                    str_month = format!("0{}", str_month);
                }
                if str_day.len() < 2 {
                    str_day = format!("0{}", str_day);
                }

                let str_date: String = format!("{}{}", str_month, str_day);
                map_dates.insert(str_date, dt);
            }

            let people: Vec<utils::Person> = utils::happy_birthday(&map_dates, &birthdays);
            if people.len() == 0 {
                println!("Nobody is celebrating recently. Enjoy a quiet day!");
                process::exit(0);
            }

            println!(
                "==> There are {} people celebrating recently:",
                people.len()
            );
            for person in people {
                println!(
                    "{} {} on {}.",
                    person.firstname,
                    person.lastname,
                    map_dates
                        .get(&person.birthday)
                        .unwrap()
                        .format("%b %e %Y")
                        .to_string()
                )
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
