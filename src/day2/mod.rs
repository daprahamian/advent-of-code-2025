use anyhow::Result;

struct Input {
    start: u64,
    end: u64,
}

impl Input {
    pub fn try_from(input: &str) -> Result<Self> {
        let mut spl = input.split("-");
        let start_str = spl.next().ok_or(anyhow::anyhow!("foo"))?.to_string();
        let end_str = spl.next().ok_or(anyhow::anyhow!("bar"))?.to_string();

        let start = start_str.parse::<u64>()?;
        let end = end_str.parse::<u64>()?;

        Ok(Self { start, end })
    }
}

fn is_repeating_by_factor(input: u64, num_digits: u32, factor_digits: u32) -> bool {
    if num_digits % factor_digits != 0 {
        return false;
    }

    let mask = 10u64.pow(num_digits / factor_digits);
    let masked_v = input % mask;
    let mut n = input;
    while n > 0 {
        if n % mask != masked_v {
            return false;
        }
        n /= mask;
    }
    return true;
}

fn is_repeating(input: u64) -> bool {
    let num_digits = input.ilog10() + 1;
    for factor in 2..=num_digits {
        if is_repeating_by_factor(input, num_digits, factor) {
            return true;
        }
    }
    return false;
}

fn part_1(inputs: &Vec<&str>) -> u64 {
    let parsed_inputs = inputs.iter().filter_map(|x| Input::try_from(x).ok());

    let mut output = 0u64;
    for input in parsed_inputs {
        for num in input.start..input.end {
            let num_digits = num.ilog10() + 1;
            if is_repeating_by_factor(num, num_digits, 2) {
                output += num;
            }
        }
    }
    output
}

fn part_2(inputs: &Vec<&str>) -> u64 {
    let parsed_inputs = inputs.iter().filter_map(|x| Input::try_from(x).ok());

    let mut output = 0u64;
    for input in parsed_inputs {
        for num in input.start..input.end {
            if is_repeating(num) {
                output += num;
            }
        }
    }
    output
}

pub fn day2() {
    let input_str = std::fs::read_to_string("src/day2/input.txt").expect("File should be present");
    let input = input_str.trim().split(",").collect::<Vec<_>>();
    let result1 = part_1(&input);
    println!("Day 2 Part 1: {result1}");
    let result2 = part_2(&input);
    println!("Day 2 Part 2: {result2}");
}
