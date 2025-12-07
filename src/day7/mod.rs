fn part1(_input: &Vec<&str>) -> u64 {
    0
}

fn part2(_input: &Vec<&str>) -> u64 {
    0
}

pub fn day7() {
    let input_str = std::fs::read_to_string("src/day7/input.txt").expect("File should be present");
    let input = input_str.trim().split("\n").collect::<Vec<_>>();
    let result1 = part1(&input);
    println!("Day 7 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 7 Part 2: {result2}");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part_1() {
        assert!(true)
    }

    #[test]
    fn test_part_2() {
        assert!(true)
    }
}
