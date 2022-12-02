pub fn day_1_1(input: &str) -> anyhow::Result<u64> {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .map(|line| line.parse::<u64>().expect("failed to parse line to u64"))
                .fold(0, |acc, elem| acc + elem)
        })
        .max()
        .ok_or(anyhow::anyhow!("failed to get max value"))
}

pub fn day_1_2(input: &str) -> anyhow::Result<u64> {
    let mut sums: Vec<u64> = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .map(|line| line.parse::<u64>().expect("failed to parse line to u64"))
                .fold(0, |acc, elem| acc + elem)
        })
        .collect();

    sums.sort_unstable();

    Ok(sums.iter().rev().take(3).sum())
}

#[cfg(test)]
mod test {
    use super::day_1_1;
    use super::day_1_2;

    const INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day_1_1() {
        assert_eq!(day_1_1(INPUT).expect("failed to run 1.1"), 24000);
    }

    #[test]
    fn test_day_1_2() {
        assert_eq!(day_1_2(INPUT).expect("failed to run 1.2"), 45000);
    }
}
