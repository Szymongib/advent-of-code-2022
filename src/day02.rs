use std::str::FromStr;

use anyhow::Ok;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u64)]
enum GameMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u64)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl GameMove {
    fn game_result(self, other: GameMove) -> GameResult {
        if self == other {
            return GameResult::Draw;
        }

        match (self, other) {
            (GameMove::Rock, GameMove::Paper) => GameResult::Loss,
            (GameMove::Rock, GameMove::Scissors) => GameResult::Win,
            (GameMove::Paper, GameMove::Rock) => GameResult::Win,
            (GameMove::Paper, GameMove::Scissors) => GameResult::Loss,
            (GameMove::Scissors, GameMove::Rock) => GameResult::Loss,
            (GameMove::Scissors, GameMove::Paper) => GameResult::Win,
            _ => panic!("unexpected game!"),
        }
    }

    fn move_for_result(self, res: GameResult) -> GameMove {
        if res == GameResult::Draw {
            return self;
        }

        match (self, res) {
            (GameMove::Rock, GameResult::Loss) => GameMove::Scissors,
            (GameMove::Rock, GameResult::Win) => GameMove::Paper,
            (GameMove::Paper, GameResult::Loss) => GameMove::Rock,
            (GameMove::Paper, GameResult::Win) => GameMove::Scissors,
            (GameMove::Scissors, GameResult::Loss) => GameMove::Paper,
            (GameMove::Scissors, GameResult::Win) => GameMove::Rock,
            (m, r) => panic!("unexpected move + result combination: {:?} - {:?}", m, r),
        }
    }
}

impl FromStr for GameMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => GameMove::Rock,
            "B" | "Y" => GameMove::Paper,
            "C" | "Z" => GameMove::Scissors,
            s => anyhow::bail!("unexpected move: {}", s),
        })
    }
}

impl FromStr for GameResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            s => anyhow::bail!("unexpected result: {}", s),
        })
    }
}

pub fn day_2_1(input: &str) -> anyhow::Result<u64> {
    let games: Vec<(GameMove, GameMove)> = input
        .split("\n")
        .map(|line| {
            let (m1, m2) = line.split_once(" ").expect("expected to elements in line");
            (
                GameMove::from_str(m1).unwrap(),
                GameMove::from_str(m2).unwrap(),
            )
        })
        .collect();

    let score = games
        .iter()
        .map(|(m1, m2)| m2.game_result(*m1) as u64 + *m2 as u64)
        .sum();

    Ok(score)
}

pub fn day_2_2(input: &str) -> anyhow::Result<u64> {
    let games: Vec<(GameMove, GameResult)> = input
        .split("\n")
        .map(|line| {
            let (m1, m2) = line.split_once(" ").expect("expected to elements in line");
            (
                GameMove::from_str(m1).unwrap(),
                GameResult::from_str(m2).unwrap(),
            )
        })
        .collect();

    let score = games
        .iter()
        .map(|(m1, res)| {
            let m2 = m1.move_for_result(*res);
            m2 as u64 + *res as u64
        })
        .sum();

    Ok(score)
}

#[cfg(test)]
mod test {
    use super::day_2_1;
    use super::day_2_2;

    const INPUT: &str = r"A Y
B X
C Z";

    #[test]
    fn test_1() {
        assert_eq!(day_2_1(INPUT).expect("failed to run 1"), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(day_2_2(INPUT).expect("failed to run 2"), 12);
    }
}
