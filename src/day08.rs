use super::util::parse_to_digit_grid;

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let trees_grid = parse_to_digit_grid(input);

    let rows_count = trees_grid.len();
    let cols_count = trees_grid[0].len();

    // Get all outer trees
    let mut visable_trees = (2 * rows_count) + (2 * (cols_count - 2));

    let mut max_top = trees_grid[0].clone();
    let mut max_left: Vec<u32> = trees_grid.iter().map(|r| r[0]).collect();
    let mut visable = vec![vec![false; cols_count]; rows_count];

    for row in 1..rows_count - 1 {
        for col in 1..cols_count - 1 {
            visable[row][col] =
                trees_grid[row][col] > max_top[col] || trees_grid[row][col] > max_left[row];
            max_top[col] = max_top[col].max(trees_grid[row][col]);
            max_left[row] = max_left[row].max(trees_grid[row][col]);
        }
    }

    let mut max_bottom = trees_grid[rows_count - 1].clone();
    let mut max_right: Vec<u32> = trees_grid.iter().map(|r| r[cols_count - 1]).collect();

    for row in (1..rows_count - 1).rev() {
        for col in (1..cols_count - 1).rev() {
            visable[row][col] = visable[row][col]
                || trees_grid[row][col] > max_bottom[col]
                || trees_grid[row][col] > max_right[row];
            max_bottom[col] = max_bottom[col].max(trees_grid[row][col]);
            max_right[row] = max_right[row].max(trees_grid[row][col]);
        }
    }

    for row in visable {
        for is_visable in row {
            if is_visable {
                visable_trees += 1;
            }
        }
    }

    Ok(visable_trees)
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let trees_grid = parse_to_digit_grid(input);

    let rows_count = trees_grid.len();
    let cols_count = trees_grid[0].len();

    let mut max_score = 0;

    for i in 0..rows_count {
        for k in 0..cols_count {
            let up = view_distance(&trees_grid, (i as i32, k as i32), (-1, 0));
            let down = view_distance(&trees_grid, (i as i32, k as i32), (1, 0));
            let left = view_distance(&trees_grid, (i as i32, k as i32), (0, -1));
            let right = view_distance(&trees_grid, (i as i32, k as i32), (0, 1));

            max_score = max_score.max(up * down * left * right);
        }
    }

    Ok(max_score as usize)
}

fn view_distance(trees: &[Vec<u32>], pos: (i32, i32), direction: (i32, i32)) -> u32 {
    let mut dest = destination(pos, direction);
    let mut view_dist = 0;
    while valid_dest(trees, dest) {
        view_dist += 1;
        if trees[dest.0 as usize][dest.1 as usize] >= trees[pos.0 as usize][pos.1 as usize] {
            break;
        }

        dest = destination(dest, direction);
    }

    view_dist
}

fn destination(pos: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    let row = pos.0 + direction.0;
    let col = pos.1 + direction.1;
    (row, col)
}

fn valid_dest(trees: &[Vec<u32>], dest: (i32, i32)) -> bool {
    if dest.0 < 0 || dest.0 >= trees.len() as i32 || dest.1 < 0 || dest.1 >= trees[0].len() as i32 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 21);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 8);
    }
}
