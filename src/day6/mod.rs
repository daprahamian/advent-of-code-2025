#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

impl Operation {
    fn operate(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mult => lhs * rhs,
        }
    }

    fn identity(&self) -> u64 {
        match self {
            Operation::Add => 0,
            Operation::Mult => 1,
        }
    }
}

impl From<char> for Operation {
    fn from(input: char) -> Self {
        match input {
            '+' => Operation::Add,
            '*' => Operation::Mult,
            _ => panic!("Unknown op '{input}'"),
        }
    }
}
impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        match input {
            "+" => Operation::Add,
            "*" => Operation::Mult,
            _ => panic!("Unknown op '{input}'"),
        }
    }
}

struct Computation {
    total: u64,
    op: Operation,
}

impl Computation {
    fn push(&mut self, v: u64) {
        self.total = self.op.operate(self.total, v);
    }
}

impl From<&str> for Computation {
    fn from(input: &str) -> Self {
        let op = Operation::from(input);
        let total = op.identity();
        Self { total, op }
    }
}

#[derive(Debug)]
struct WeirdComputation {
    op: Operation,
    bay_count: usize,
    bays: Vec<Vec<char>>,
}

impl WeirdComputation {
    fn new(op_str: char, bay_count: usize) -> Self {
        let bays = (0..bay_count)
            .into_iter()
            .map(|_| Vec::new())
            .collect::<Vec<_>>();
        Self {
            bays,
            bay_count,
            op: op_str.into(),
        }
    }

    fn push(&mut self, input: &str) {
        for (i, c) in input.chars().enumerate() {
            self.bays[i].push(c);
        }
    }

    fn compute(&self) -> u64 {
        self.bays
            .iter()
            .map(|bay| {
                bay.iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            })
            .fold(self.op.identity(), |acc, n| self.op.operate(acc, n))
    }
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let last_line = lines.next_back().unwrap();
    let mut ops = last_line
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|op| op.into())
        .collect::<Vec<Computation>>();
    for line in lines {
        for (i, v) in line
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .enumerate()
        {
            ops[i].push(v);
        }
    }
    ops.iter().map(|op| op.total).sum()
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let last_line = format!("{} ", lines.next_back().unwrap());
    let mut ops = last_line
        .chars()
        .fold(Vec::new(), |mut acc, c| {
            match c {
                '+' | '*' => acc.push((c, 0usize)),
                ' ' => {
                    let l = acc.last_mut().unwrap();
                    l.1 += 1;
                }
                _ => panic!("Invalid character '{c}'"),
            };
            acc
        })
        .into_iter()
        .map(|(c, i)| WeirdComputation::new(c, i))
        .collect::<Vec<_>>();

    for line in lines {
        let mut start = 0usize;
        for op in ops.iter_mut() {
            let end = start + op.bay_count;
            let segment = &line[start..end];
            op.push(segment);
            start += op.bay_count + 1;
        }
    }
    ops.iter().map(|op| op.compute()).sum()
}

pub fn day6() {
    let input = std::fs::read_to_string("src/day6/input.txt").expect("File should be present");
    let result1 = part1(&input);
    println!("Day 6 Part 1: {result1}");
    let result2 = part2(&input);
    println!("Day 6 Part 2: {result2}");
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  \
";
    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 3263827);
    }
}
