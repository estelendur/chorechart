#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use chrono::{Date, DateTime, Local, Weekday};
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

#[derive(Debug, Serialize)]
struct AbstractChore {
    name: String,
    recurrence: Recurrence,
    people: Vec<String>
}

#[derive(Debug, Serialize)]
struct ManifestChore {
    name: String,
    day: Weekday,
    person: String
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

fn main() {
    let (month, header_days) = build_calendar(Local::now().date());
    println!("{:?}", month);
    let tera = compile_templates!("./templates/*");
    let context = json!({
        "month": &month,
        "header_days": &header_days,
    });
    let render = tera.render("calendar.html", &context).unwrap();
    let mut f = File::create("./output.html").unwrap();
    f.write_all(render.as_bytes()).unwrap();
}
