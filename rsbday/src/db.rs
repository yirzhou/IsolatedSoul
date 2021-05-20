extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};

use crate::records;

const QUERY_CREATE_TABLE: &str = "CREATE TABLE birthday (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    firstname TEXT NOT NULL,
    lastname  TEXT NOT NULL,
    birthday  TEXT NOT NULL,
    platform  TEXT NOT NULL
)";

const QUERY_INSERT: &str = "INSERT INTO birthday (
        firstname, lastname, birthday, platform) 
        values (?1, ?2, ?3, ?4)";

pub struct DB {
    conn: Connection,
    file_path: String,
}

impl DB {
    pub fn create_table(&self) -> Result<usize> {
        return self.conn.execute(QUERY_CREATE_TABLE, NO_PARAMS);
    }

    pub fn populate_table_tx(&mut self, records: &Vec<records::Row>) -> Result<usize> {
        let mut num_rows_affected: usize = 0;
        let tx = self.conn.transaction().unwrap();
        for record in records {
            if tx
                .execute(
                    QUERY_INSERT,
                    &[
                        &record.firstname,
                        &record.lastname,
                        &record.birthday,
                        &record.platform,
                    ],
                )
                .unwrap()
                == 1
            {
                num_rows_affected += 1
            };
        }
        if num_rows_affected != records.len() {
            println!(
                "Row numbers do not match: {} rows affected vs {} records",
                num_rows_affected,
                records.len()
            );
        } else {
            println!("{} rows have been inserted.", num_rows_affected);
        }
        tx.commit().unwrap();
        Ok(num_rows_affected)
    }

    pub fn populate_table(&self, records: &Vec<records::Row>) -> Result<usize> {
        let mut num_rows_affected: usize = 0;
        for record in records {
            if self
                .conn
                .execute(
                    QUERY_INSERT,
                    &[
                        &record.firstname,
                        &record.lastname,
                        &record.birthday,
                        &record.platform,
                    ],
                )
                .unwrap()
                == 1
            {
                num_rows_affected += 1
            };
        }
        if num_rows_affected != records.len() {
            println!(
                "Row numbers do not match: {} rows affected vs {} records",
                num_rows_affected,
                records.len()
            );
        } else {
            println!("{} rows have been inserted.", num_rows_affected);
        }
        Ok(num_rows_affected)
    }

    pub fn fetch_all(&self) -> Option<Vec<records::Row>> {
        let mut stmt = self
            .conn
            .prepare("SELECT firstname, lastname, birthday, platform FROM birthday")
            .unwrap();

        let mut birthdays = Vec::new();

        let birthday_iter = stmt
            .query_map(NO_PARAMS, |row| {
                Ok(records::Row {
                    firstname: row.get(0)?,
                    lastname: row.get(1)?,
                    birthday: row.get(2)?,
                    platform: row.get(3)?,
                })
            })
            .unwrap();

        for birthday in birthday_iter {
            birthdays.push(birthday.unwrap());
        }
        Some(birthdays)
    }
}

pub fn new(file_path: &str) -> DB {
    let conn = Connection::open(file_path).unwrap();
    return DB {
        conn: conn,
        file_path: file_path.to_string(),
    };
}
