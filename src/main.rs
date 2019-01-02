#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use chrono::{Date, Local, Weekday};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize)]
enum Recurrence {
    Daily,
    Weekly(Weekday),
    Monthly
}

impl<'de> Deserialize<'de> for Recurrence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;

        match s.as_ref() {
            "Daily" => Ok(Recurrence::Daily),
            "Monthly" => Ok(Recurrence::Monthly),
            other => {
                match Weekday::from_str(&other) {
                    Ok(weekday) => Ok(Recurrence::Weekly(weekday)),
                    _ => Err(Error::custom(format!("not a valid day: {}", other)))
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AbstractChore {
    name: String,
    recurrence: Recurrence,
    weight: u32,
    people: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ManifestChore {
    name: String,
    person: String,
    weight: u32,
}

impl ManifestChore {
    fn from_abstract(chore: AbstractChore, person: String) -> ManifestChore {
        ManifestChore {
            name: chore.name,
            person: person,
            weight: chore.weight,
        }
    }
}

#[derive(Debug, Serialize)]
struct Day {
    day: Weekday,
    date_string: String,
    chores: Vec<ManifestChore>
}

fn build_calendar(start: Date<Local>) -> (Vec<Day>, Vec<Weekday>) {
    let mut month = vec![];
    let mut header_days = vec![];
    let mut now = start.clone();
    while month.len() < 28 {
        // If this fails, chrono isn't doing its job
        let day = Weekday::from_str(&format!("{}", now.format("%a"))).unwrap();
        if header_days.len() < 7 {
            header_days.push(day);
        }
        let new_day = Day {
            day,
            date_string: format!("{}", now.format("%b %d")),
            chores: vec![]
        };
        month.push(new_day);
        now = now.succ();
    }
    (month, header_days)
}

fn read_chores(data_file: &Path) -> Vec<AbstractChore> {
    let mut chores = vec![];
    let csv_rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_path(data_file)
        .unwrap();
    let mut iter = csv_rdr.into_records();
    while let Some(record_result) = iter.next() {
        match record_result {
            Ok(record) => {
                match record.deserialize(None) {
                    Ok(chore) => chores.push(chore),
                    Err(e) => println!("chore error: {:?}", e),
                }
            },
            Err(e) => println!("record error: {:?}", e),
        }
    }
    chores
}

fn fill_calendar(month: Vec<Day>, abstract_chores: Vec<AbstractChore>) -> Vec<Day> {
    month
}

fn main() {
    let (month, header_days) = build_calendar(Local::now().date());
    let abstract_chores = read_chores(Path::new("./data.txt"));

    let tera = compile_templates!("./templates/*");
    let context = json!({
        "month": &month,
        "header_days": &header_days,
    });
    let render = tera.render("calendar.html", &context).unwrap();
    let mut f = File::create("./output.html").unwrap();
    f.write_all(render.as_bytes()).unwrap();
}
