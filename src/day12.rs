use anyhow::Ok;
use std::collections::{HashSet, VecDeque};

use super::util::parse_to_char_grid;

fn square_height(c: char) -> i64 {
    match c {
        'a'..='z' => (c as i64) - ('a' as i64) + 1,
        'S' => 1,
        'E' => ('z' as i64) - ('a' as i64) + 1,
        c => panic!("invalid char: {}", c),
    }
}

// We could do Dikstra but BFS is fine and I am lazy...
fn find_shortest_path_bfs(grid: &mut [Vec<char>], starting_pos: &[(usize, usize)]) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    for s in starting_pos {
        visited.insert(*s);
        queue.push_back(*s);
    }

    let mut steps = 1;
    let mut next_step = VecDeque::new();

    while let Some((row, col)) = queue.pop_front() {
        let height = square_height(grid[row][col]);

        for (r_step, c_step) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let next_pos = (row as i64 + r_step, col as i64 + c_step);

            // Out of bounds
            if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 >= grid.len() as i64
                || next_pos.1 >= grid[0].len() as i64
            {
                continue;
            }
            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
            // Already visited
            if visited.contains(&next_pos) {
                continue;
            }
            // Incorrect height
            let step_h = square_height(grid[next_pos.0][next_pos.1]);
            if step_h - height > 1 {
                continue;
            }

            visited.insert(next_pos);
            if grid[next_pos.0][next_pos.1] == 'E' {
                return steps;
            }

            next_step.push_back(next_pos);
        }

        if queue.len() == 0 {
            queue.append(&mut next_step);
            next_step.clear();
            steps += 1;
        }
    }

    unreachable!("failed to reach the end")
}

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let mut grid = parse_to_char_grid(input);

    let start_pos = grid
        .iter()
        .enumerate()
        .filter_map(|(row_id, row)| {
            let col_id = row.iter().position(|c| *c == 'S')?;
            Some((row_id, col_id))
        })
        .next()
        .unwrap();

    Ok(find_shortest_path_bfs(&mut grid, &[start_pos]))
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let mut grid = parse_to_char_grid(input);

    let start_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(row_id, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_id, c)| {
                    if *c == 'S' || *c == 'a' {
                        Some((row_id, col_id))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(find_shortest_path_bfs(&mut grid, &start_pos))
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 31);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 29);
    }
}
