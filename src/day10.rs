use std::str::FromStr;

enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "addx" => Ok(Instruction::Addx(parts[1].parse::<i64>().unwrap())),
            "noop" => Ok(Instruction::Noop),
            s => unreachable!("unexpected instruction: {}", s),
        }
    }
}

pub fn task_1(input: &str) -> anyhow::Result<i64> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut x = 1;
    let mut cycles = 0;
    let mut signals_sum = 0;

    for ins in instructions {
        match ins {
            Instruction::Addx(n) => {
                cycles += 2;

                // Addx instruction takes two cycles to execute so we can go
                // from 59->61, therefore we subtract reminder of division by 2
                // to make sure we are always check even cycle.
                let cyc_check = cycles - cycles % 2;
                if (cyc_check - 20) % 40 == 0 {
                    println!("Counting on cycle: {}", cyc_check);
                    signals_sum += cyc_check * x;
                }
                x += n;
            }
            Instruction::Noop => {
                cycles += 1;
                if (cycles - 20) % 40 == 0 {
                    println!("Counting on cycle (noop): {}", cycles);
                    signals_sum += cycles * x;
                }
            }
        }
    }

    Ok(signals_sum)
}

pub fn task_2(input: &str) -> anyhow::Result<String> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut screen = vec!['.'; 240];

    let mut x: i64 = 1;
    let mut cycles: usize = 0;

    for ins in instructions {
        match ins {
            Instruction::Addx(n) => {
                for _i in 0..2 {
                    cycles += 1;
                    draw_pixels(&mut screen, x, cycles);
                }
                x += n;
            }
            Instruction::Noop => {
                cycles += 1;
                draw_pixels(&mut screen, x, cycles);
            }
        }
    }

    let mut out = String::new();
    for i in 0..(240 / 40) {
        out.push_str(&screen[i * 40..(i + 1) * 40].iter().collect::<String>());
        out.push('\n');
    }

    println!("{}", out);

    Ok(out)
}

fn draw_pixels(screen: &mut Vec<char>, x: i64, cycles: usize) {
    let c = (cycles as i64 - 1) % 40;

    for xp in x - 1..x + 2 {
        if xp == c {
            screen[cycles - 1] = '#';
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 13140);
    }

    const P2_OUT: &str = r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), P2_OUT.to_string());
    }
}
