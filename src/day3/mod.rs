fn find_biggest_char_in_str(line: &str) -> (usize, char) {
    let mut chars_iter = line.chars().enumerate();
    let (mut greatest_index, mut greatest) = chars_iter.next().unwrap();

    for (index, char) in chars_iter {
        if char > greatest {
            greatest = char;
            greatest_index = index;
        }
    }
    (greatest_index, greatest)
}

fn find_joltage_n(line: &str, joltage: usize) -> u64 {
    let mut chars = Vec::with_capacity(joltage);
    let mut start = 0usize;
    let first_end_index = line.len() - joltage + 1;
    for end in first_end_index..=line.len() {
        let substr = line.get(start..end).unwrap();
        let (index, c) = find_biggest_char_in_str(substr);
        start += index + 1;
        chars.push(c);
    }
    chars.iter().collect::<String>().parse::<u64>().unwrap()
}

fn part1(input: &Vec<&str>) -> u64 {
    input
        .iter()
        .map(|line| find_joltage_n(line, 2))
        .fold(0u64, |acc, n| acc + n)
}

fn part2(input: &Vec<&str>) -> u64 {
    input
        .iter()
        .map(|line| find_joltage_n(line, 12))
        .fold(0u64, |acc, n| acc + n)
}

pub fn day3() {
    let input_str = std::fs::read_to_string("src/day3/input.txt").expect("File should be present");
    let input = input_str.trim().split("\n").collect::<Vec<_>>();
    let result1 = part1(&input);
    println!("Day 3 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 3 Part 2: {result2}");
}

mod test {

    #[test]
    fn find_joltage_2() {
        assert_eq!(super::find_joltage_n("987654321111111", 2), 98);
        assert_eq!(super::find_joltage_n("811111111111119", 2), 89);
        assert_eq!(super::find_joltage_n("234234234234278", 2), 78);
        assert_eq!(super::find_joltage_n("818181911112111", 2), 92);
    }

    #[test]
    fn find_joltage_12() {
        assert_eq!(super::find_joltage_n("987654321111111", 12), 987654321111);
        assert_eq!(super::find_joltage_n("811111111111119", 12), 811111111119);
        assert_eq!(super::find_joltage_n("234234234234278", 12), 434234234278);
        assert_eq!(super::find_joltage_n("818181911112111", 12), 888911112111);
    }
}
