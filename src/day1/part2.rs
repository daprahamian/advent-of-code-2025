const MAX: u64 = 100;

struct Change {
    magnitude: u64,
    direction: i64,
}

impl Change {
    fn from(input: &String) -> Option<Change> {
        let Some(direction_str) = input.get(0..1) else {
            return None;
        };
        let direction = if direction_str == "L" { -1 } else { 1 };
        let Some(magnitude) = input.get(1..).and_then(|x| x.parse::<u64>().ok()) else {
            return None;
        };

        Some(Self {
            magnitude,
            direction,
        })
    }

    fn full_rotations(&self) -> u64 {
        self.magnitude / MAX
    }
    fn partial_rotation(&self) -> i64 {
        (self.magnitude.rem_euclid(MAX)) as i64 * self.direction
    }
}

struct Answer {
    count: u64,
    pos: i64,
}

impl Answer {
    fn new(start: i64) -> Self {
        Self {
            count: 0,
            pos: start,
        }
    }

    fn apply_change(&mut self, change: &Change) {
        self.count += change.full_rotations();

        let partial_rot = change.partial_rotation();
        let new_pos = self.pos + partial_rot;
        let normalized = new_pos.rem_euclid(MAX as i64);

        if self.pos != 0 && (new_pos <= 0 || new_pos >= 100) {
            self.count += 1;
        }
        self.pos = normalized;
    }
}

pub fn part2(starting_pos: i64, input: &Vec<String>) -> u64 {
    let mut answer = Answer::new(starting_pos);

    for change in input.iter().filter_map(Change::from) {
        answer.apply_change(&change)
    }

    answer.count
}
