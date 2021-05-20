extern crate chrono;

use chrono::offset::LocalResult;
use chrono::prelude::*;
use csv::Reader;
use serde::Deserialize;
use std::error::Error;

const PLATFORM_WECHAT: &str = "wechat";
const PLATFORM_FACEBOOK: &str = "facebook";

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Row {
    pub firstname: String,
    pub lastname: String,
    pub birthday: String,
    pub platform: String,
}

pub struct RecordPreprocessor {
    pub file_path: String,
}

impl RecordPreprocessor {
    pub fn read_data(&self) -> Result<Vec<Row>, Box<dyn Error>> {
        let mut birthdays: Vec<Row> = Vec::new();
        match Reader::from_path(&self.file_path) {
            Ok(mut rdr) => {
                for result in rdr.records() {
                    let record: Row = result?.deserialize(None)?;
                    birthdays.push(record);
                }
            }
            Err(e) => {
                println!("error opening file: {:?}", e)
            }
        }
        Ok(birthdays)
    }
}
