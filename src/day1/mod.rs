mod part1;
mod part2;

pub fn day1() {
    let input_str = std::fs::read_to_string("src/day1/input.txt").expect("File should be present");
    let input = input_str
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let result1 = part1::part1(50, &input);
    let result2 = part2::part2(50, &input);
    println!("DAY 1 part 1: {result1}");
    println!("DAY 1 part 2: {result2}");
}
