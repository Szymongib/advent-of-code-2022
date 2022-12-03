use std::collections::{HashSet, HashMap};

pub fn task_1(input: &str) -> anyhow::Result<u64> {
    let priority_sum: u64 = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let c = find_item_in_both_parts(line);
            get_item_priority(c)
        }).sum();

    Ok(priority_sum)
}

fn find_item_in_both_parts(s: &str) -> char {
    assert!(s.len()%2 == 0);

    let p1 = &s[..s.len()/2];
    let p2 = &s[s.len()/2..];

    let mut elems = HashSet::new();
    for c in p1.chars() {
        elems.insert(c);
    }

    for c in p2.chars() {
        if elems.contains(&c) {
            return c;
        }
    }
    unreachable!("expected exactly one duplicate!");
}

fn get_item_priority(item: char) -> u64 {
    if item.is_lowercase() {
        (item as u64) - 96
    } else {
        (item as u64) - 64 + 26
    }
}

pub fn task_2(input: &str) -> anyhow::Result<u64> {
     let rucksacks: Vec<&str> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect();

    let sum = rucksacks.chunks(3).map(|group| {
        get_item_priority(
            find_id_in_group(group)
        )
    }).sum();

     Ok(sum)
}

fn find_id_in_group(group: &[&str]) -> char {
    assert!(group.len() == 3);

    let mut items: HashMap<char, usize> = HashMap::new();
    for (i, sack) in group.iter().enumerate() {
        for c in sack.chars() {
            let entry = items.entry(c).or_insert(0);
            if *entry == i {
                *entry += 1;
            }
        }
    }
    for (k, v) in items {
        if v == 3 {
            return k;
        }
    }
    unreachable!("expected exctly one id in group of 3!");
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 157);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 70);
    }
}

