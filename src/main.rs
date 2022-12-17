use std::{env, fmt::Display, process};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod util;

pub fn run_task<T, O>(func: T, day: &str, task: &str, input: &str)
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
        [day @ "1", task @ "1"] => {
            run_task(day01::day_1_1, day, task, include_str!("../inputs/01.txt"))
        }
        [day @ "1", task @ "2"] => {
            run_task(day01::day_1_2, day, task, include_str!("../inputs/01.txt"))
        }
        [day @ "2", task @ "1"] => {
            run_task(day02::day_2_1, day, task, include_str!("../inputs/02.txt"))
        }
        [day @ "2", task @ "2"] => {
            run_task(day02::day_2_2, day, task, include_str!("../inputs/02.txt"))
        }
        [day @ "3", task @ "1"] => {
            run_task(day03::task_1, day, task, include_str!("../inputs/03.txt"))
        }
        [day @ "3", task @ "2"] => {
            run_task(day03::task_2, day, task, include_str!("../inputs/03.txt"))
        }
        [day @ "4", task @ "1"] => {
            run_task(day04::task_1, day, task, include_str!("../inputs/04.txt"))
        }
        [day @ "4", task @ "2"] => {
            run_task(day04::task_2, day, task, include_str!("../inputs/04.txt"))
        }
        [day @ "5", task @ "1"] => {
            run_task(day05::task_1, day, task, include_str!("../inputs/05.txt"))
        }
        [day @ "5", task @ "2"] => {
            run_task(day05::task_2, day, task, include_str!("../inputs/05.txt"))
        }
        [day @ "6", task @ "1"] => {
            run_task(day06::task_1, day, task, include_str!("../inputs/06.txt"))
        }
        [day @ "6", task @ "2"] => {
            run_task(day06::task_2, day, task, include_str!("../inputs/06.txt"))
        }
        [day @ "7", task @ "1"] => {
            run_task(day07::task_1, day, task, include_str!("../inputs/07.txt"))
        }
        [day @ "7", task @ "2"] => {
            run_task(day07::task_2, day, task, include_str!("../inputs/07.txt"))
        }
        [day @ "8", task @ "1"] => {
            run_task(day08::task_1, day, task, include_str!("../inputs/08.txt"))
        }
        [day @ "8", task @ "2"] => {
            run_task(day08::task_2, day, task, include_str!("../inputs/08.txt"))
        }
        [day @ "9", task @ "1"] => {
            run_task(day09::task_1, day, task, include_str!("../inputs/09.txt"))
        }
        [day @ "9", task @ "2"] => {
            run_task(day09::task_2, day, task, include_str!("../inputs/09.txt"))
        }
        [day @ "10", task @ "1"] => {
            run_task(day10::task_1, day, task, include_str!("../inputs/10.txt"))
        }
        [day @ "10", task @ "2"] => {
            run_task(day10::task_2, day, task, include_str!("../inputs/10.txt"))
        }
        [day @ "11", task @ "1"] => {
            run_task(day11::task_1, day, task, include_str!("../inputs/11.txt"))
        }
        [day @ "11", task @ "2"] => {
            run_task(day11::task_2, day, task, include_str!("../inputs/11.txt"))
        }
        [day @ "12", task @ "1"] => {
            run_task(day12::task_1, day, task, include_str!("../inputs/12.txt"))
        }
        [day @ "12", task @ "2"] => {
            run_task(day12::task_2, day, task, include_str!("../inputs/12.txt"))
        }
        [day @ "13", task @ "1"] => {
            run_task(day13::task_1, day, task, include_str!("../inputs/13.txt"))
        }
        [day @ "13", task @ "2"] => {
            run_task(day13::task_2, day, task, include_str!("../inputs/13.txt"))
        }
        [day @ "14", task @ "1"] => {
            run_task(day14::task_1, day, task, include_str!("../inputs/14.txt"))
        }
        [day @ "14", task @ "2"] => {
            run_task(day14::task_2, day, task, include_str!("../inputs/14.txt"))
        }
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
    use crate::day04;
    use crate::day05;
    use crate::day06;
    use crate::day07;
    use crate::day08;
    use crate::day09;
    use crate::day10;
    use crate::day11;
    use crate::day12;
    use crate::day13;
    use crate::day14;
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

        assert_eq!(
            day04::task_1(include_str!("../inputs/04.txt")).expect("day 4_1 failed"),
            read_output(out_path("04.1.txt"))
        );
        assert_eq!(
            day04::task_2(include_str!("../inputs/04.txt")).expect("day 4_2 failed"),
            read_output(out_path("04.2.txt"))
        );

        assert_eq!(
            day05::task_1(include_str!("../inputs/05.txt")).expect("day 5_1 failed"),
            read_output::<String, String>(out_path("05.1.txt"))
        );
        assert_eq!(
            day05::task_2(include_str!("../inputs/05.txt")).expect("day 5_2 failed"),
            read_output::<String, String>(out_path("05.2.txt"))
        );

        assert_eq!(
            day06::task_1(include_str!("../inputs/06.txt")).expect("day 6_1 failed"),
            read_output(out_path("06.1.txt"))
        );
        assert_eq!(
            day06::task_2(include_str!("../inputs/06.txt")).expect("day 6_2 failed"),
            read_output(out_path("06.2.txt"))
        );

        assert_eq!(
            day07::task_1(include_str!("../inputs/07.txt")).expect("day 7_1 failed"),
            read_output(out_path("07.1.txt"))
        );
        assert_eq!(
            day07::task_2(include_str!("../inputs/07.txt")).expect("day 7_2 failed"),
            read_output(out_path("07.2.txt"))
        );

        assert_eq!(
            day08::task_1(include_str!("../inputs/08.txt")).expect("day 8_1 failed"),
            read_output(out_path("08.1.txt"))
        );
        assert_eq!(
            day08::task_2(include_str!("../inputs/08.txt")).expect("day 8_2 failed"),
            read_output(out_path("08.2.txt"))
        );

        assert_eq!(
            day09::task_1(include_str!("../inputs/09.txt")).expect("day 9_1 failed"),
            read_output(out_path("09.1.txt"))
        );
        assert_eq!(
            day09::task_2(include_str!("../inputs/09.txt")).expect("day 9_2 failed"),
            read_output(out_path("09.2.txt"))
        );

        assert_eq!(
            day10::task_1(include_str!("../inputs/10.txt")).expect("day 10_1 failed"),
            read_output(out_path("10.1.txt"))
        );
        assert_eq!(
            day10::task_2(include_str!("../inputs/10.txt")).expect("day 10_2 failed"),
            read_output::<_, String>(out_path("10.2.txt"))
        );

        assert_eq!(
            day11::task_1(include_str!("../inputs/11.txt")).expect("day 11_1 failed"),
            read_output(out_path("11.1.txt"))
        );
        assert_eq!(
            day11::task_2(include_str!("../inputs/11.txt")).expect("day 11_2 failed"),
            read_output(out_path("11.2.txt"))
        );

        assert_eq!(
            day12::task_1(include_str!("../inputs/12.txt")).expect("day 12_1 failed"),
            read_output(out_path("12.1.txt"))
        );
        assert_eq!(
            day12::task_2(include_str!("../inputs/12.txt")).expect("day 12_2 failed"),
            read_output(out_path("12.2.txt"))
        );

        assert_eq!(
            day13::task_1(include_str!("../inputs/13.txt")).expect("day 13_1 failed"),
            read_output(out_path("13.1.txt"))
        );
        assert_eq!(
            day13::task_2(include_str!("../inputs/13.txt")).expect("day 13_2 failed"),
            read_output(out_path("13.2.txt"))
        );

        assert_eq!(
            day14::task_1(include_str!("../inputs/14.txt")).expect("day 14_1 failed"),
            read_output(out_path("14.1.txt"))
        );
        assert_eq!(
            day14::task_2(include_str!("../inputs/14.txt")).expect("day 14_2 failed"),
            read_output(out_path("14.2.txt"))
        );
    }
}
