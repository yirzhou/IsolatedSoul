#[macro_use]
extern crate clap;
extern crate colored;
use clap::{App, Arg, SubCommand};
use colored::*;

use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::fs::remove_file;
use std::path::Path;
use std::process;

mod db;
mod records;
mod utils;

const SUBCMD_RESET: &str = "reset";
const ARG_DAYS: &str = "day-range";
const ARG_CSV_PATH: &str = "csv-path";
const ARG_DB_PATH: &str = "db-path";

fn main() {
    // Command-line argument parsing
    let matches = App::new("rsbday")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Check out the people you love who are having birthdays recently!")
        .arg(
            Arg::with_name(ARG_DAYS)
                .short("r")
                .long(ARG_DAYS)
                .value_name("DAY RANGE")
                .help("Sets a custom range of days")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_DB_PATH)
                .long(ARG_DB_PATH)
                .value_name("DB")
                .help("The path to your database file")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name(SUBCMD_RESET)
                .about("Reset your sqlite database.")
                .arg(
                    Arg::with_name(ARG_CSV_PATH)
                        .long(ARG_CSV_PATH)
                        .value_name("CSV")
                        .help("The path to your CSV file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    let db_path = matches.value_of(ARG_DB_PATH).unwrap();
    // Resetting database...
    let mut csv_path: &str = "";
    let mut reset_required: bool = false;
    if let Some(matches) = matches.subcommand_matches(SUBCMD_RESET) {
        println!("{}", "Resetting database...".yellow());
        reset_required = true;
        if let Some(csv_val) = matches.value_of(ARG_CSV_PATH) {
            csv_path = csv_val;
            if Path::new(db_path).exists() {
                remove_file(db_path).unwrap();
            }
        }
    }
    let mut db = db::new(db_path);
    if reset_required {
        let preprocessor = records::RecordPreprocessor {
            file_path: csv_path.to_string(),
        };
        let records = preprocessor.read_data().unwrap();
        db.create_table().unwrap();
        db.populate_table_tx(&records).unwrap();
        println!("{}", "Database has been reset!\n".green())
    }

    // Retrieve day range...
    // The default is 7 days (one week).
    let mut day_range = 7;
    if let Some(arg_day_val) = matches.value_of(ARG_DAYS) {
        day_range = arg_day_val.parse::<i64>().unwrap();
    }

    // Fetch, compute, and return people celebrating recently...
    if let Some(birthdays) = db.fetch_all() {
        if birthdays.len() > 0 {
            let dt = chrono::Local::today();
            println!(
                "{}",
                format!("Today is {}.\n", dt.format("%a %b %e").to_string()).bright_green()
            );

            // Get people having birthdays between today - 7 days and today + 7 days.
            let mut map_dates: HashMap<String, Date<Local>> = HashMap::new();
            for offset in -day_range..day_range {
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
                println!(
                    "{}",
                    "Nobody is celebrating recently. Enjoy a quiet day!".green()
                );
                process::exit(0);
            }

            println!(
                "{}",
                format!(
                    "==> There are {} people celebrating recently:",
                    people.len()
                )
                .yellow()
            );
            for person in people {
                println!(
                    "{}",
                    format!(
                        "{} {} on {}.",
                        person.firstname,
                        person.lastname,
                        map_dates
                            .get(&person.birthday)
                            .unwrap()
                            .format("%b %e %Y")
                            .to_string()
                    )
                    .bright_blue()
                )
            }
        } else {
            println!("{}", "Nothing for today!".green());
            process::exit(0);
        }
    } else {
        println!("{}", "Nothing for today!".green());
        process::exit(0);
    }
    process::exit(0);
}
