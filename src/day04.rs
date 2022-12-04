use std::ops::Range;

fn str_to_ranges(s: &str) -> (Range<u64>, Range<u64>) {
    let (r1, r2) = s.split_once(",").unwrap();
    (str_to_range(r1), str_to_range(r2))
}

fn str_to_range(s: &str) -> Range<u64> {
    let (start, end) = s
        .split_once("-")
        .map(|(s, e)| {
            (
                s.parse::<u64>().expect("expected unsigned int"),
                e.parse::<u64>().expect("expected unsigned int"),
            )
        })
        .expect("expected - split to succeed");
    start..end
}

fn range_contains(a: &Range<u64>, b: &Range<u64>) -> bool {
    b.start >= a.start && b.end <= a.end
}

fn ranges_separate(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.end < b.start || b.end < a.start
}

pub fn task_1(input: &str) -> anyhow::Result<u64> {
    let sum = input
        .split("\n")
        .map(|line| {
            let (r1, r2) = str_to_ranges(line);
            if range_contains(&r1, &r2) || range_contains(&r2, &r1) {
                1
            } else {
                0
            }
        })
        .sum();

    Ok(sum)
}

pub fn task_2(input: &str) -> anyhow::Result<u64> {
    let sum = input
        .split("\n")
        .map(|line| {
            let (r1, r2) = str_to_ranges(line);
            if ranges_separate(&r1, &r2) {
                0
            } else {
                1
            }
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 4);
    }
}
