use std::fs;

fn crack_safe(starting_pos: i64, input: Vec<String>) -> u64 {
    let mut count = 0;
    let mut pos: i64 = starting_pos;

    let t = input.iter().filter_map(|line| {
        let Some(direction) = line.get(0..1) else {
            return None;
        };
        line.get(1..)
            .and_then(|x| x.parse::<i64>().ok())
            .map(|q| if direction == "L" { -q } else { q })
    });

    for change in t {
        pos += change;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            count += 1
        }
    }
    count
}

pub fn day1() {
    let input_str = fs::read_to_string("src/day1/input.txt").expect("File should be present");
    let input = input_str
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let result = crack_safe(50, input);
    println!("DAY 1: {result}");
}
