use std::{collections::BTreeMap, str::FromStr};

fn position_from_str(s: &str) -> (isize, isize) {
    let (x, y) = s.split_once(", ").unwrap();
    (
        x.strip_prefix("x=").unwrap().parse().unwrap(),
        y.strip_prefix("y=").unwrap().parse().unwrap(),
    )
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: isize,
    y: isize,
    // Distance to the closest beacon
    dist: isize,
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Sensor at ").unwrap();
        let (sensor_p, beacon_p) = s.split_once(": closest beacon is at ").unwrap();

        let sensor_position = position_from_str(sensor_p);
        let beacon_position = position_from_str(beacon_p);
        Ok(Self {
            x: sensor_position.0,
            y: sensor_position.1,
            dist: manhattan_distance(sensor_position, beacon_position),
        })
    }
}

fn manhattan_distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn find_empty_ranges(sensors: &[Sensor], row: isize) -> Vec<(isize, isize)> {
    let ranges = sensors
        .iter()
        .filter_map(|s| {
            let d = (s.y - row).abs();
            (d <= s.dist).then_some((s.x - (s.dist - d), s.x + (s.dist - d)))
        })
        .collect::<std::collections::BinaryHeap<_>>();

    flatten_ranges(&ranges.into_sorted_vec())
}

fn flatten_ranges(ranges: &[(isize, isize)]) -> Vec<(isize, isize)> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut flatten = vec![ranges[0]];
    for r in ranges.iter().skip(1) {
        let last = flatten.len() - 1;

        // If ranges overlap, merge them
        match flatten[last].0 <= r.0 && r.0 <= flatten[last].1 {
            true => flatten[last].1 = flatten[last].1.max(r.1),
            _ => flatten.push(*r),
        }
    }

    flatten
}

pub fn task_1(input: &str) -> anyhow::Result<isize> {
    let sensors = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    let empty = find_empty_ranges(&sensors, 2000000)
        .iter()
        .map(|(start, end)| end - start)
        .sum();

    Ok(empty)
}

pub fn task_2(input: &str) -> anyhow::Result<isize> {
    let sensors = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Sensor>>();

    let mut m1 = BTreeMap::<isize, isize>::new();
    let mut m2 = BTreeMap::<isize, isize>::new();

    for s in &sensors {
        *m1.entry(s.y - s.x + s.dist + 1).or_default() += 1;
        *m1.entry(s.y - s.x - s.dist - 1).or_default() += 1;
        *m2.entry(s.x + s.y + s.dist + 1).or_default() += 1;
        *m2.entry(s.x + s.y - s.dist - 1).or_default() += 1;
    }

    let m1 = m1
        .into_iter()
        .filter_map(|(k, v)| if v > 1 { Some(k) } else { None });

    let m2 = m2
        .into_iter()
        .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
        .collect::<Vec<_>>();

    for a in m1 {
        for b in &m2 {
            let x = (b - a) / 2;
            let y = (a + b) / 2;
            if x.min(y) <= 0 || x.max(y) >= 4000000 {
                continue;
            }

            if sensors
                .iter()
                .all(|s| manhattan_distance((s.x, s.y), (x, y)) > s.dist)
            {
                return Ok(x * 4000000 * y);
            }
        }
    }

    unreachable!("expected to find solution");
}
