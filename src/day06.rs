use std::collections::{HashMap, HashSet};

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let mut used = HashMap::new();

    let chars: Vec<char> = input.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        *used.entry(c).or_insert(0) += 1;

        if used.len() == 4 {
            return Ok(i + 1);
        }
        if i >= 3 {
            let e = used.get_mut(&chars[i - 3]).unwrap();
            *e -= 1;
            if *e == 0 {
                used.remove(&chars[i - 3]);
            }
        }
    }

    unreachable!("expected to find begining");
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let mut used = HashMap::new();

    let chars: Vec<char> = input.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        *used.entry(c).or_insert(0) += 1;
        if used.len() == 14 {
            return Ok(i + 1);
        }
        if i >= 13 {
            let e = used.get_mut(&chars[i - 13]).unwrap();
            *e -= 1;
            if *e == 0 {
                used.remove(&chars[i - 13]);
            }
        }
    }

    unreachable!("expected to find begining");
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_2: &str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_3: &str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_4: &str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_5: &str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 7);
        assert_eq!(task_1(INPUT_2).expect("failed to run 1"), 5);
        assert_eq!(task_1(INPUT_3).expect("failed to run 1"), 6);
        assert_eq!(task_1(INPUT_4).expect("failed to run 1"), 10);
        assert_eq!(task_1(INPUT_5).expect("failed to run 1"), 11);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 19);
        assert_eq!(task_2(INPUT_2).expect("failed to run 2"), 23);
        assert_eq!(task_2(INPUT_3).expect("failed to run 2"), 23);
        assert_eq!(task_2(INPUT_4).expect("failed to run 2"), 29);
        assert_eq!(task_2(INPUT_5).expect("failed to run 2"), 26);
    }
}
