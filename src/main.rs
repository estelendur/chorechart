#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod models;

use crate::models::*;

fn main() {
    let data: ChoreData = serde_json::from_str(include!("../data.txt"))
        .expect("Invalid data format!");
    let mut week: Vec<ChoreDay> = ChoreDay::new_week(data.people);
    println!("{:?}", week[0]);
    println!("{:?}", week[1]);
    println!("---------------------------");
    for n in 0..7 {
        let day = &mut week[n];
        for pile in data.daily.iter() {
            day.add_chores(&pile, n);
        }
    }
    for pile in data.weekly.iter() {
        println!("weekly pile {:?}", pile);
        let tasks = pile.tasks.clone();
        // No weekly chores on Friday
        let weekly_fraction = (tasks.len() as f32 / 6.0).ceil() as usize;
        let weekly_parts = tasks.chunks(weekly_fraction);
        let mut weekly_parts = weekly_parts.into_iter().enumerate();
        println!("weekly chunks {:?}", weekly_parts);
        while let Some((n, chunk)) = weekly_parts.next() {
            println!("chunk {}", n);
            // We're pretending day 0 is sunday
            let n = match n {
                5 => 6,
                _ => n
            };
            let day = &mut week[n];
            let temp_pile = ChorePile {
                day: None,
                people: pile.people.clone(),
                tasks: chunk.to_vec()
            };
            println!("adding chores {:?} to day {}", chunk, n);
            day.add_chores(&pile, n);
        }
    }

    println!("{:?}", week[0]);
    println!("{:?}", week[1]);
}
