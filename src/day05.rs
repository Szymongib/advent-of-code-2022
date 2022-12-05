use std::collections::VecDeque;

use anyhow::Ok;

fn parse_stacks(s: &str) -> Vec<VecDeque<char>> {
    let lines: Vec<&str> = s.split("\n").collect();

    let mut stacks = vec![VecDeque::new(); lines[0].len() / 4 + 1];

    for line in lines.iter().take(lines.len() - 1) {
        let mut i = 1;
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c == ' ' {
                continue;
            }
            stacks[i].push_front(c);
        }
    }

    return stacks;
}

pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn apply_9000(&self, stacks: &mut Vec<VecDeque<char>>) {
        for _c in 0..self.count {
            let e = stacks[self.from - 1].pop_back().unwrap();
            stacks[self.to - 1].push_back(e);
        }
    }

    fn apply_9001(&self, stacks: &mut Vec<VecDeque<char>>) {
        let mut transfer = VecDeque::with_capacity(self.count);

        for _ in 0..self.count {
            transfer.push_front(stacks[self.from - 1].pop_back().unwrap());
        }
        for t in transfer {
            stacks[self.to - 1].push_back(t);
        }
    }
}

fn parse_moves(s: &str) -> Vec<Move> {
    s.split("\n")
        .map(|line| {
            let (count, rem) = line
                .strip_prefix("move ")
                .unwrap()
                .split_once(" from ")
                .unwrap();
            let (from, to) = rem.split_once(" to ").unwrap();
            Move {
                count: count.parse().unwrap(),
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
            }
        })
        .collect()
}

pub fn task_1(input: &str) -> anyhow::Result<String> {
    // TODO: refactor common part
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    let moves = parse_moves(moves);

    for m in moves {
        m.apply_9000(&mut stacks);
    }

    let mut out = String::new();
    for s in stacks {
        out.push(*s.back().unwrap());
    }
    Ok(out)
}

pub fn task_2(input: &str) -> anyhow::Result<String> {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    let moves = parse_moves(moves);

    for m in moves {
        m.apply_9001(&mut stacks);
    }

    let mut out = String::new();
    for s in stacks {
        out.push(*s.back().unwrap());
    }
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), String::from("CMZ"));
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), "MCD");
    }
}
