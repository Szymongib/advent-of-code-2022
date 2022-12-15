use std::{collections::VecDeque, str::FromStr};

struct Calculation {
    op1: Operand,
    op2: Operand,
    operation: Operation,
}

impl Calculation {
    fn calculate(&self, old: i64) -> i64 {
        let op1 = match self.op1 {
            Operand::Old => old,
            Operand::Value(v) => v,
        };
        let op2 = match self.op2 {
            Operand::Old => old,
            Operand::Value(v) => v,
        };
        match self.operation {
            Operation::Multiply => op1 * op2,
            Operation::Add => op1 + op2,
        }
    }
}

impl FromStr for Calculation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        let op1 = match parts[0] {
            "old" => Operand::Old,
            s => Operand::Value(s.parse::<i64>().unwrap()),
        };
        let op2 = match parts[2] {
            "old" => Operand::Old,
            s => Operand::Value(s.parse::<i64>().unwrap()),
        };
        let operation = match parts[1] {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            s => unreachable!("unexpected operation: {}", s),
        };
        Ok(Calculation {
            op1,
            op2,
            operation,
        })
    }
}

enum Operand {
    Old,
    Value(i64),
}

enum Operation {
    Multiply,
    Add,
}

struct MonkeyLogic {
    calculation: Calculation,
    test_div: i64,
    true_dest: usize,
    false_dest: usize,
}

struct MonkeySimulation {
    monkeys: Vec<MonkeyLogic>,
    item_queueus: Vec<VecDeque<i64>>,

    inspect_conut: Vec<usize>,
    div_factor: i64,
}

impl MonkeySimulation {
    fn run_simulation(&mut self, rounds: usize, worry_div: Option<i64>) {
        for _ in 0..rounds {
            for (i, monkey) in self.monkeys.iter().enumerate() {
                while let Some(item) = self.item_queueus[i].pop_front() {
                    self.inspect_conut[i] += 1;
                    let mut new_worry = monkey.calculation.calculate(item);
                    if let Some(worry_div) = worry_div {
                        new_worry = new_worry / worry_div;
                    } else {
                        new_worry = new_worry % self.div_factor;
                    }

                    if new_worry % monkey.test_div == 0 {
                        self.item_queueus[monkey.true_dest].push_back(new_worry);
                    } else {
                        self.item_queueus[monkey.false_dest].push_back(new_worry);
                    }
                }
            }
        }
    }
}

fn parse_monkeys(input: &str) -> MonkeySimulation {
    let data: Vec<(VecDeque<i64>, MonkeyLogic)> = input
        .split("\n\n")
        .map(|monkey| {
            let lines = monkey.lines().collect::<Vec<_>>();
            assert_eq!(lines.len(), 6);
            let items = lines[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<VecDeque<_>>();
            let calculation =
                Calculation::from_str(lines[2].strip_prefix("  Operation: new = ").unwrap())
                    .unwrap();
            let test = lines[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let true_dest = lines[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let false_dest = lines[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            (
                items,
                MonkeyLogic {
                    calculation,
                    test_div: test,
                    true_dest,
                    false_dest,
                },
            )
        })
        .collect();

    let mut monkeys = Vec::new();
    let mut item_queueus = Vec::new();

    for (items, logic) in data {
        monkeys.push(logic);
        item_queueus.push(items);
    }

    let inspect_conut = vec![0; monkeys.len()];
    let div_factor = monkeys.iter().map(|m| m.test_div).product();

    MonkeySimulation {
        monkeys,
        item_queueus,
        inspect_conut,
        div_factor,
    }
}

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let mut simulation = parse_monkeys(input);

    simulation.run_simulation(20, Some(3));

    let mut counts = simulation.inspect_conut.clone();
    counts.sort_unstable();

    let monkey_business = counts.iter().rev().take(2).product::<usize>();

    Ok(monkey_business)
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let mut simulation = parse_monkeys(input);

    simulation.run_simulation(10000, None);

    let mut counts = simulation.inspect_conut.clone();
    counts.sort_unstable();

    let monkey_business = counts.iter().rev().take(2).product::<usize>();

    Ok(monkey_business)
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 10605);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 2713310158);
    }
}
