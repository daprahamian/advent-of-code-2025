mod day1;

use std::collections::HashMap;

fn main() {
    let mut days: HashMap<u64, Box<dyn Fn()>> = HashMap::new();
    days.insert(1u64, Box::new(day1::day1));

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
