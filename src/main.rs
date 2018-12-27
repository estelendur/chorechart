extern crate csv;
extern crate chrono;

use chrono::{Date, DateTime, Local, Weekday};
use std::str::FromStr;

#[derive(Debug)]
enum Recurrence {
    Daily,
    Weekly(Weekday),
    Monthly
}

#[derive(Debug)]
struct AbstractChore {
    name: String,
    recurrence: Recurrence,
    people: Vec<String>
}

#[derive(Debug)]
struct ManifestChore {
    name: String,
    day: Weekday,
    person: String
}

#[derive(Debug)]
struct Day {
    day: Weekday,
    date: Date<Local>,
    chores: Vec<ManifestChore>
}

fn build_calendar(start: Date<Local>) -> Vec<Day> {
    let mut month = vec![];
    let mut now = start.clone();
    while month.len() < 28 {
        // If this fails, chrono isn't doing its job
        let day = Weekday::from_str(&format!("{}", now.format("%a"))).unwrap();
        let new_day = Day {
            day,
            date: now,
            chores: vec![]
        };
        month.push(new_day);
        now = now.succ();
    }
    return month
}

fn main() {
    let month = build_calendar(Local::now().date());
    println!("{:?}", month)
}
