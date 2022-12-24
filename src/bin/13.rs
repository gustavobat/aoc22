use std::cmp::Ordering;
use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
enum Value {
    Number(u32),
    Array(Box<Vec<Value>>),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Value, Self::Err> {
        let mut num = None;
        let mut vecs = vec![vec![]];
        let push_num = |num: Option<u32>, vecs: &mut Vec<Vec<Value>>| {
            let depth = vecs.len() - 1;
            if let Some(num) = num {
                vecs[depth].push(Value::Number(num));
            }
            None
        };
        for c in s.chars() {
            match c {
                '[' => vecs.push(vec![]),
                ']' => {
                    num = push_num(num, &mut vecs);
                    let depth = vecs.len() - 1;
                    let vec = vecs.pop().unwrap();
                    vecs[depth - 1].push(Value::Array(Box::new(vec)));
                }
                ' ' => (),
                ',' => num = push_num(num, &mut vecs),
                '0'..='9' => num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap()),
                _ => ()
            };
        }
        Ok(vecs[0].pop().unwrap())
    }
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, other) {
            (Value::Number(l_val), Value::Number(r_val)) => (*l_val).cmp(r_val),
            (Value::Array(l_list), Value::Array(r_list)) => {
                let mut i = 0;
                while i < l_list.len() && i < r_list.len() {
                    match &l_list[i].cmp(&r_list[i]) {
                        Ordering::Equal => {}
                        other => return *other,
                    };
                    i += 1;
                }
                l_list.len().cmp(&r_list.len())
            }
            (l, Value::Number(v)) => l.cmp(&&Value::Array(Box::new(vec![Value::Number(*v)]))),
            (Value::Number(v), l) => Value::Array(Box::new(vec![Value::Number(*v)])).cmp(l),
        }
    }
}

fn main() -> Result<()> {
    let mut packets: Vec<Value> = std::fs::read_to_string("./data/13.input")?
        .lines()
        .filter(|line| line.len() > 0)
        .filter_map(|line| line.parse::<Value>().ok())
        .collect();

    let acc_indices_of_right_order_pairs: i32 = packets
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| (i + 1) as i32)
        .sum();
    println!("Sum of the indices of the pairs in right order: {acc_indices_of_right_order_pairs}");

    let divider_packets : Vec<Value> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    packets.push("[[2]]".parse().unwrap());
    packets.push("[[6]]".parse().unwrap());
    packets.sort();
    let decoder_key : i32 = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(i, _)| (i + 1) as i32)
        .product();
    println!("Decoder key: {decoder_key}");

    Ok(())
}