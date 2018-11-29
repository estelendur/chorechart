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
        let tasks = pile.tasks.clone();
        // No weekly chores on Friday
        let weekly_fraction = (tasks.len() as f32 / 6.0).ceil() as usize;
        let weekly_parts = tasks.chunks(weekly_fraction);
        let mut weekly_parts = weekly_parts.into_iter().enumerate();
        while let Some((n, chunk)) = weekly_parts.next() {
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
            day.add_chores(&temp_pile, n);
        }
    }
    let mut weekly_on_day = data.weekly_on_day.into_iter().enumerate();
    while let Some((n, pile)) = weekly_on_day.next() {
        if let Some(day_index) = pile.day {
            let day = &mut week[day_index as usize];
            day.add_chores(&pile, n);
        }
    }
    let monthly_piles = data.monthly.iter();
    for pile in monthly_piles {
        let mut tasks = pile.tasks.iter();
        for day in week.clone() {
            for mut chunk in day.chunks {
                if pile.people.contains(&chunk.person) {
                    let chunk_len = chunk.chores.len();
                    for n in 0..chunk_len {
                        if chunk.chores[n] == "" {
                            if let Some(task) = tasks.next() {
                                std::mem::replace(&mut chunk.chores[n], task.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", week[0]);
    println!("{:?}", week[1]);
}
