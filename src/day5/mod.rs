use anyhow::{Result, anyhow};
use std::ops::Range;

struct FoodDb {
    fresh_foods: Vec<Range<u64>>,
    inventory: Vec<u64>,
}

impl FoodDb {
    fn try_new(input: &str) -> Result<Self> {
        let mut ranges = Vec::new();
        let mut foods = Vec::new();

        let mut hit_middle = false;
        for line in input.split("\n") {
            match (line, hit_middle) {
                ("", _) => hit_middle = true,
                (l, false) => {
                    let mut chars = l.split("-");
                    let start_str = chars.next().ok_or(anyhow!("Line missing start index"))?;
                    let end_str = chars.next().ok_or(anyhow!("Line missing end index"))?;

                    let start = start_str.parse::<u64>()?;
                    let end = end_str.parse::<u64>()?;

                    ranges.push(start..(end + 1));
                }
                (l, true) => {
                    let food = l.parse::<u64>()?;
                    foods.push(food);
                }
            }
        }

        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        Ok(Self {
            fresh_foods: ranges,
            inventory: foods,
        })
    }

    fn is_inventory_item_fresh(&self, food: u64) -> bool {
        self.fresh_foods
            .iter()
            .find(|range| range.contains(&food))
            .is_some()
    }

    fn count_fresh_inventory(&self) -> u64 {
        self.inventory
            .iter()
            .filter(|food| self.is_inventory_item_fresh(**food))
            .count() as u64
    }

    fn count_fresh_foods(&self) -> u64 {
        self.fresh_foods
            .iter()
            .fold((0u64, 0u64), |(count, start), range| {
                if start >= range.end {
                    (count, start)
                } else {
                    let max_start = if start > range.start {
                        start
                    } else {
                        range.start
                    };
                    (count + (range.end - max_start), range.end)
                }
            })
            .0
    }
}

fn part1(input: &str) -> u64 {
    let db = FoodDb::try_new(input).expect("should properly parse food db");
    db.count_fresh_inventory()
}

fn part2(input: &str) -> u64 {
    let db = FoodDb::try_new(input).expect("should properly parse food db");
    db.count_fresh_foods()
}

pub fn day5() {
    let input = std::fs::read_to_string("src/day5/input.txt").expect("File should be present");
    let result1 = part1(&input);
    println!("Day 5 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 5 Part 2: {result2}");
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 14);
    }
}
