use anyhow::Result;
use std::str::FromStr;
use aoc22::read_one_every_double_linebreak;

#[derive(Clone, Debug)]
enum Operation {
    Addition,
    Multiplication,
    Square
}

#[derive(Clone, Debug)]
struct Monkey {
    items : Vec<u64>,
    operation : (Operation, Option<u64>),
    test : (u64, usize, usize),
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<_> = s.lines().map(|l| l.split(": ").last().unwrap()).collect();
        let m = Monkey {
            items: l[1].split(", ").map(|n| n.parse().unwrap()).collect(),
            operation: {
                let op: Vec<_> = l[2].rsplit_once("= ").unwrap().1.split(' ').collect();
                match op[2] {
                    "old" => (Operation::Square, None),
                    num => match (op[1], num.parse::<u64>().unwrap()) {
                        ("+", n) => (Operation::Addition, Some(n)),
                        ("*", n) => (Operation::Multiplication, Some(n)),
                        _ => unreachable!(),
                    },
                }
            },
            test: ( l[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
                    l[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                    l[5].rsplit_once(' ').unwrap().1.parse().unwrap() ),
        };
        Ok(m)
    }
}

fn process_round(
    monkeys: &mut Vec<Monkey>,
    items_inspectioned_per_monkey: &mut Vec<u64>,
    worry_relief : fn(u64, &mut Vec<Monkey>) -> u64
) {
    let mut cloned_monkeys = monkeys.clone();
    for i in 0..monkeys.len() {
        let monkey = &cloned_monkeys[i];
        items_inspectioned_per_monkey[i] += monkey.items.len() as u64;
        for item in monkey.items.clone() {
            let mut worry_level = match &monkey.operation.0 {
                Operation::Addition => item + monkey.operation.1.unwrap(),
                Operation::Multiplication => item * monkey.operation.1.unwrap(),
                Operation::Square => item * item,
            };
            worry_level = worry_relief(worry_level, monkeys);

            let (dividend, monkey_true, monkey_false) = monkey.test;
            if worry_level % dividend == 0 {
                monkeys[monkey_true].items.push(worry_level);
            } else {
                monkeys[monkey_false].items.push(worry_level);
            }
        }
        monkeys[i].items.clear();
        cloned_monkeys = monkeys.clone();
    }
    *monkeys = cloned_monkeys;
}

fn solve(
    mut monkeys: Vec<Monkey>,
    n_rounds : usize,
    worry_relief : fn(u64, &mut Vec<Monkey> ) -> u64
) -> u64 {

    let mut items_inspectioned_per_monkey : Vec<u64>= vec![0; monkeys.len()];
    for _ in 0..n_rounds {
        process_round(&mut monkeys, &mut items_inspectioned_per_monkey, worry_relief);
    }

    items_inspectioned_per_monkey.sort_by(|a, b| b.cmp(a));
    items_inspectioned_per_monkey
        .iter()
        .take(2)
        .product::<u64>()
}

fn worry_relief_part_1(level : u64, _: &mut Vec<Monkey>)  -> u64 {
    level / 3
}

fn worry_relief_part_2(level : u64, monkeys: &mut Vec<Monkey>)  -> u64 {
    let common_denominator : u64 = monkeys
        .iter()
        .map(|m| m.test.0)
        .product();
    level % common_denominator
}

fn main () -> Result<()> {
    let monkeys = read_one_every_double_linebreak::<Monkey>("./data/11.input")?;

    println!("Level of monkey business in part 1: {:?}",
             solve(monkeys.clone(), 20, worry_relief_part_1));
    println!("Level of monkey business in part 2: {:?}",
             solve(monkeys.clone(), 10_000, worry_relief_part_2));

    Ok(())
}