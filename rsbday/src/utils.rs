use crate::records;

use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
pub struct Person {
    pub firstname: String,
    pub lastname: String,
    pub birthday: String,
}

pub fn get_month_date(md: &String) -> (u32, u32) {
    let month_str = md[0..2].to_string();
    let date_str = md[2..].to_string();

    (
        month_str.parse::<u32>().unwrap(),
        date_str.parse::<u32>().unwrap(),
    )
}

pub fn happy_birthday(
    map_dates: &HashMap<String, Date<Local>>,
    birthdays: &Vec<records::Row>,
) -> Vec<Person> {
    let mut people: Vec<Person> = Vec::new();

    for birthday in birthdays {
        if map_dates.contains_key(&birthday.birthday) {
            people.push(Person {
                firstname: birthday.firstname.clone(),
                lastname: birthday.lastname.clone(),
                birthday: birthday.birthday.clone(),
            })
        }
    }
    people
}
