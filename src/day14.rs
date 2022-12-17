#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Rock,
    Air,
    Abyss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RockLine {
    start: (usize, usize),
    end: (usize, usize),
}

impl RockLine {
    pub fn new_from_point(start: (usize, usize), end: (usize, usize)) -> Self {
        // We assume that line is always horizontal or vertical.
        Self {
            start: (start.0.min(end.0), start.1.min(end.1)),
            end: (start.0.max(end.0), start.1.max(end.1)),
        }
    }
}

fn parse_rock_lines(line: &str) -> Vec<RockLine> {
    let points: Vec<(usize, usize)> = line
        .split(" -> ")
        .map(|point| {
            let (start, end) = point.split_once(",").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let mut rock_lines = Vec::new();

    for point_slice in points.windows(2) {
        rock_lines.push(RockLine::new_from_point(point_slice[0], point_slice[1]));
    }

    rock_lines
}

fn get_at(grid: &Vec<Vec<Tile>>, pos: (i32, i32)) -> Tile {
    if pos.0 < 0 || pos.0 >= grid[0].len() as i32 || pos.1 >= grid.len() as i32 {
        return Tile::Abyss;
    }

    let pos = (pos.0 as usize, pos.1 as usize);

    grid[pos.1][pos.0]
}

fn run_simulation(rock_lines: &[RockLine], max_x: usize, max_y: usize) -> usize {
    let mut grid = vec![vec![Tile::Air; max_x + 1]; max_y + 1];

    for rl in rock_lines {
        for x in rl.start.0..=rl.end.0 {
            for y in rl.start.1..=rl.end.1 {
                grid[y][x] = Tile::Rock;
            }
        }
    }

    let mut sand_in_abyss = false;
    let mut sand_down = 0;

    let mut sand_in_air = (500, 0);

    let move_options = vec![(0, 1), (-1, 1), (1, 1)];

    while !sand_in_abyss {
        let mut moved = false;
        for opt in &move_options {
            let next_pos = (sand_in_air.0 + opt.0, sand_in_air.1 + opt.1);
            match get_at(&grid, next_pos) {
                Tile::Sand | Tile::Rock => {
                    continue;
                }
                Tile::Air => {
                    moved = true;
                    sand_in_air = next_pos;
                    break;
                }
                Tile::Abyss => {
                    sand_in_abyss = true;
                    break;
                }
            }
        }
        if sand_in_abyss {
            break;
        }
        if !moved {
            grid[sand_in_air.1 as usize][sand_in_air.0 as usize] = Tile::Sand;
            sand_down += 1;
            if grid[0][500] != Tile::Air {
                break;
            }

            sand_in_air = (500, 0);
        }
    }

    sand_down
}

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let rock_lines = input
        .lines()
        .flat_map(|line| parse_rock_lines(line))
        .collect::<Vec<RockLine>>();

    let max_x = rock_lines
        .iter()
        .map(|line| line.start.0.max(line.end.0))
        .max()
        .unwrap()
        .max(500);

    let max_y = rock_lines
        .iter()
        .map(|line| line.start.1.max(line.end.1))
        .max()
        .unwrap();

    Ok(run_simulation(&rock_lines, max_x, max_y))
}

// TODO: I suppose this could be done more optimally without needing to run the
// whole simulation.
pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let mut rock_lines = input
        .lines()
        .flat_map(|line| parse_rock_lines(line))
        .collect::<Vec<RockLine>>();

    let max_x = rock_lines
        .iter()
        .map(|line| line.start.0.max(line.end.0))
        .max()
        .unwrap()
        .max(500);

    let max_y = rock_lines
        .iter()
        .map(|line| line.start.1.max(line.end.1))
        .max()
        .unwrap();

    let max_y = max_y + 2;

    // This is pretty ugly workaround to just add some value to max_x to make
    // sure we do not overflow on the right side, otherwise we would need to
    // support some kind of dynamic resizing of the grid.
    // We use 500+max_y because we should not be able to go over that since sand
    // will not be able to fall down priro to reaching that point (at least I think so :shrug:).
    let max_x = max_x.max(500 + max_y);

    rock_lines.push(RockLine {
        start: (0, max_y),
        end: (max_x, max_y),
    });

    Ok(run_simulation(&rock_lines, max_x, max_y))
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 24);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 93);
    }
}
