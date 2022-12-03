use std::{env, fmt::Display, process};

mod day01;
mod day02;
mod day03;
mod util;

pub fn run_task<T, O>(func: T, day: u16, task: u16, input: &str)
where
    T: Fn(&str) -> anyhow::Result<O>,
    O: Display,
{
    println!("Running Day {} task {}", day, task);

    let out = func(input).expect("failed to run");

    println!("Output: {}", out);
}

fn main() {
    println!("Running Advent of code 2022!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Invalid arguments. Provide [DAY] [TASK] as an arguments.");
        process::exit(1);
    }

    let args: [&str; 2] = [&args[1], &args[2]];

    match args {
        ["1", "1"] => run_task(day01::day_1_1, 1, 1, include_str!("../inputs/01.txt")),
        ["1", "2"] => run_task(day01::day_1_2, 1, 2, include_str!("../inputs/01.txt")),
        ["2", "1"] => run_task(day02::day_2_1, 2, 1, include_str!("../inputs/02.txt")),
        ["2", "2"] => run_task(day02::day_2_2, 2, 2, include_str!("../inputs/02.txt")),
        ["3", "1"] => run_task(day03::task_1, 3, 1, include_str!("../inputs/03.txt")),
        ["3", "2"] => run_task(day03::task_2, 3, 2, include_str!("../inputs/03.txt")),
        [day, task] => {
            println!("Invalid arguments, day: {}, task: {}", day, task);
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day01::{day_1_1, day_1_2};
    use crate::day02::{day_2_1, day_2_2};
    use crate::day03;
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;

    fn read_output<P: AsRef<Path>, T: FromStr>(out_file: P) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        fs::read_to_string(out_file)
            .expect("failed to read out file")
            .parse()
            .expect("failed to parse output")
    }

    fn out_path(file: &str) -> String {
        format!("outputs/{}", file)
    }

    #[test]
    fn test_with_inputs() {
        assert_eq!(
            day_1_1(include_str!("../inputs/01.txt")).expect("day 1_1 failed"),
            read_output(out_path("01.1.txt"))
        );
        assert_eq!(
            day_1_2(include_str!("../inputs/01.txt")).expect("day 1_2 failed"),
            read_output(out_path("01.2.txt"))
        );

        assert_eq!(
            day_2_1(include_str!("../inputs/02.txt")).expect("day 2_1 failed"),
            read_output(out_path("02.1.txt"))
        );
        assert_eq!(
            day_2_2(include_str!("../inputs/02.txt")).expect("day 2_2 failed"),
            read_output(out_path("02.2.txt"))
        );

         assert_eq!(
            day03::task_1(include_str!("../inputs/03.txt")).expect("day 3_1 failed"),
            read_output(out_path("03.1.txt"))
        );
        assert_eq!(
            day03::task_2(include_str!("../inputs/03.txt")).expect("day 3_2 failed"),
            read_output(out_path("03.2.txt"))
        );
    }
}
