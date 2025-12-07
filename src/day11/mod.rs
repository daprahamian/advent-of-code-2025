fn part1(_input: &str) -> u64 {
    0
}

fn part2(_input: &str) -> u64 {
    0
}

pub fn day11() {
    let input = std::fs::read_to_string("src/day11/input.txt").expect("File should be present");
    let result1 = part1(&input);
    println!("Day 11 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 11 Part 2: {result2}");
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "";
    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 0);
    }
}
