use crate::records;

pub struct Person {
    pub firstname: String,
    pub lastname: String,
    pub platform: String,
}

pub fn get_month_date(md: &String) -> (u32, u32) {
    let month_str = md[0..1].to_string();
    let date_str = md[2..3].to_string();

    (
        month_str.parse::<u32>().unwrap(),
        date_str.parse::<u32>().unwrap(),
    )
}

pub fn happy_birthday(
    month_target: u32,
    date_target: u32,
    birthdays: &Vec<records::Row>,
) -> Vec<Person> {
    let mut people: Vec<Person> = Vec::new();

    for birthday in birthdays {
        let (month, date) = get_month_date(&birthday.birthday);
        if month == month_target && date == date_target {
            people.push(Person {
                firstname: birthday.firstname.clone(),
                lastname: birthday.lastname.clone(),
                platform: birthday.platform.clone(),
            })
        }
    }
    people
}
