use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    distance: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_once(" ").unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance = distance.parse::<usize>().unwrap();
        Ok(Move {
            direction,
            distance,
        })
    }
}

impl Direction {
    fn apply_move(&self, pos: Position) -> Position {
        match self {
            Direction::Up => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0, pos.1 - 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

type Position = (usize, usize);

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let moves: Vec<Move> = input
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .collect();

    let mut visited = HashSet::new();

    let mut head_pos = (500, 500);
    let mut tail_pos = (500, 500);

    visited.insert((head_pos.0, head_pos.1));

    for m in moves {
        // TODO: this is done really naively, we could do it better if the
        // number of moves is large.
        for _i in 0..m.distance {
            let new_head_pos = m.direction.apply_move(head_pos);

            if let Some(move_vector) = need_to_move(new_head_pos, tail_pos) {
                tail_pos = move_by_vector(tail_pos, move_vector);
                visited.insert((tail_pos.0, tail_pos.1));
            }

            head_pos = new_head_pos;
        }
    }

    Ok(visited.len())
}

fn need_to_move(head: Position, tail: Position) -> Option<(i32, i32)> {
    let m = (head.0 as i32 - tail.0 as i32, head.1 as i32 - tail.1 as i32);
    if m.0.abs() <= 1 && m.1.abs() <= 1 {
        return None;
    }
    Some(cut_diff_to_move_vector(m))
}

fn move_by_vector(pos: Position, m: (i32, i32)) -> Position {
    let new_pos = (pos.0 as i32 + m.0, pos.1 as i32 + m.1);
    (new_pos.0 as usize, new_pos.1 as usize)
}

fn cut_diff_to_move_vector(mut m: (i32, i32)) -> (i32, i32) {
    if m.0.abs() >= 2 {
        m.0 = m.0 / 2;
    }
    if m.1.abs() >= 2 {
        m.1 = m.1 / 2;
    }
    m
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let moves: Vec<Move> = input
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .collect();

    let mut visited = HashSet::new();

    let mut head_pos = (500, 500);
    let mut tail_pos: [Position; 9] = [(500, 500); 9];

    for m in moves {
        // TODO: this is done really naively, we could do it better if the
        // number of moves is large.
        for _i in 0..m.distance {
            let new_head_pos = m.direction.apply_move(head_pos);
            let mut check_pos = new_head_pos;

            for k in 0..9 {
                if let Some(move_vector) = need_to_move(check_pos, tail_pos[k]) {
                    tail_pos[k] = move_by_vector(tail_pos[k], move_vector);
                }
                check_pos = tail_pos[k];
            }

            visited.insert((tail_pos[8].0, tail_pos[8].1));

            head_pos = new_head_pos;
        }
    }

    Ok(visited.len())
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 1);
        assert_eq!(task_2(INPUT2).expect("failed to run 2"), 36);
    }
}
