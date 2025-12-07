mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::collections::HashMap;

fn main() {
    let mut days: HashMap<u64, Box<dyn Fn()>> = HashMap::new();
    days.insert(1, Box::new(day1::day1));
    days.insert(2, Box::new(day2::day2));
    days.insert(3, Box::new(day3::day3));
    days.insert(4, Box::new(day4::day4));
    days.insert(5, Box::new(day5::day5));
    days.insert(6, Box::new(day6::day6));
    days.insert(7, Box::new(day7::day7));
    days.insert(8, Box::new(day8::day8));
    days.insert(9, Box::new(day9::day9));
    days.insert(10, Box::new(day10::day10));
    days.insert(11, Box::new(day11::day11));
    days.insert(12, Box::new(day12::day12));

    let day_arg: Option<u64> = std::env::args().nth(1).and_then(|d| d.parse::<u64>().ok());

    if let Some(day_of_code) = day_arg {
        if let Some(day) = days.get(&day_of_code) {
            day();
            return;
        }
    };

    for (_, day) in days.iter() {
        day();
    }
}
