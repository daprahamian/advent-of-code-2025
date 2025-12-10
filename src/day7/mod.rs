#[derive(Clone)]
enum BeamLine {
    START,
    TACHYON(u64),
    SPLITTER,
    SPACE,
}

use BeamLine::*;

impl From<char> for BeamLine {
    fn from(value: char) -> Self {
        match value {
            'S' => BeamLine::START,
            '|' => BeamLine::TACHYON(1),
            '^' => BeamLine::SPLITTER,
            _ => BeamLine::SPACE,
        }
    }
}

struct TachyonLineIterator {
    data: Vec<(Option<u64>, BeamLine, Option<u64>)>,
    splits: u64,
    index: usize,
}

impl TachyonLineIterator {
    fn new<'a>(last: Vec<BeamLine>, current: Vec<BeamLine>) -> Self {
        let mut splits = 0;
        let data = current
            .iter()
            .enumerate()
            .zip(last.iter())
            .map(|((i, me), above)| match (me, above) {
                (SPACE, TACHYON(x)) => (None, TACHYON(*x), None),
                (SPACE, START) => (None, TACHYON(1), None),
                (SPLITTER, TACHYON(x)) => {
                    splits += 1;
                    (
                        current
                            .get(i - 1)
                            .filter(|c| matches!(**c, SPACE))
                            .map(|_| *x),
                        SPLITTER,
                        current
                            .get(i + 1)
                            .filter(|c| matches!(**c, SPACE))
                            .map(|_| *x),
                    )
                }
                (x, _) => (None, x.clone(), None),
            })
            .collect::<Vec<_>>();
        Self {
            data,
            splits,
            index: 0,
        }
    }
}

impl Iterator for TachyonLineIterator {
    type Item = BeamLine;

    fn next(&mut self) -> Option<BeamLine> {
        let Some((_, current, _)) = self.data.get(self.index) else {
            return None;
        };

        let (_, _, from_prev) = self
            .index
            .checked_sub(1)
            .and_then(|i| self.data.get(i))
            .unwrap_or(&(None, SPACE, None));
        let (_, _, from_next) = self
            .data
            .get(self.index + 1)
            .unwrap_or(&(None, SPACE, None));

        self.index += 1;

        let current_branches = if let TACHYON(b) = current { *b } else { 0 };
        let total_branches =
            current_branches + from_prev.unwrap_or_default() + from_next.unwrap_or_default();

        if total_branches > 0 {
            Some(TACHYON(total_branches))
        } else {
            Some(current.clone())
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| line.chars().map(BeamLine::from).collect::<Vec<_>>());
    let Some(first_line) = lines.next() else {
        return 0;
    };
    lines
        .fold((first_line, 0), |(last, splits), current| {
            let iter = TachyonLineIterator::new(last, current);
            let new_splits = iter.splits;
            let new_line = iter.collect();
            (new_line, splits + new_splits)
        })
        .1
}

fn part2(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| line.chars().map(BeamLine::from).collect::<Vec<_>>());
    let Some(first_line) = lines.next() else {
        return 0;
    };
    let final_line = lines.fold(first_line, |last, current| {
        TachyonLineIterator::new(last, current).collect()
    });
    final_line
        .iter()
        .filter_map(|item| if let TACHYON(x) = item { Some(x) } else { None })
        .sum()
}

pub fn day7() {
    let input = std::fs::read_to_string("src/day7/input.txt").expect("File should be present");
    let result1 = part1(&input);
    println!("Day 7 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 7 Part 2: {result2}");
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 40);
    }
}
