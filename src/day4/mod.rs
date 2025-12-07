struct Grid {
    x: usize,
    y: usize,
    data: Vec<bool>,
}

impl Grid {
    fn new(input: &Vec<&str>) -> Self {
        let x = input.len();
        let y = input.get(0).map(|s| s.len()).unwrap_or_default();

        if x == 0 || y == 0 {
            panic!("This should not be possible");
        }

        let mut data = Vec::with_capacity(x * y);

        for line in input {
            for c in line.chars() {
                match c {
                    '@' => data.push(true),
                    '.' => data.push(false),
                    x => panic!("HOW did we get char '{x}'"),
                }
            }
        }

        Self { x, y, data }
    }

    fn get(&self, in_x: isize, in_y: isize) -> bool {
        let Ok(x) = usize::try_from(in_x) else {
            return false;
        };
        let Ok(y) = usize::try_from(in_y) else {
            return false;
        };
        if x >= self.x || y >= self.y {
            false
        } else {
            self.data
                .get(x * self.x + y)
                .map(|b| *b)
                .unwrap_or_default()
        }
    }

    fn get_neighbor_count(&self, x: isize, y: isize) -> u64 {
        let mut neighbors = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get(x + dx, y + dy) {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn can_be_removed(&self, x: isize, y: isize) -> bool {
        self.get(x, y) && self.get_neighbor_count(x, y) < 4
    }

    fn removal_pass(&mut self) -> u64 {
        let mut count = 0;
        for x in 0..self.x {
            for y in 0..self.y {
                if self.can_be_removed(x as isize, y as isize) {
                    self.data[x * self.x + y] = false;
                    count += 1;
                }
            }
        }
        count
    }

    fn remove_all(&mut self) -> u64 {
        let mut count = 0;
        loop {
            let new_removed = self.removal_pass();
            if new_removed == 0 {
                break;
            } else {
                count += new_removed;
            }
        }
        count
    }
}

fn part1(input: &Vec<&str>) -> u64 {
    let grid = Grid::new(input);
    let mut count = 0;

    let ix: isize = grid.x as isize;
    let iy = grid.y as isize;
    for x in 0..ix {
        for y in 0..iy {
            if grid.can_be_removed(x, y) {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &Vec<&str>) -> u64 {
    let mut grid = Grid::new(input);
    grid.remove_all()
}

pub fn day4() {
    let input_str = std::fs::read_to_string("src/day4/input.txt").expect("File should be present");
    let input = input_str.trim().split("\n").collect::<Vec<_>>();
    let result1 = part1(&input);
    println!("Day 4 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 4 Part 2: {result2}");
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_part_1() {
        let input_str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let input = input_str.split("\n").collect::<Vec<_>>();

        assert_eq!(part1(&input), 13);
    }
}
