use anyhow::Result;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Measurement {
    sensor: Position,
    beacon: Position,
}

impl FromStr for Measurement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split(|c| " =,:".contains(c));
        let sensor = {
            let col = iter.nth(3).unwrap().parse()?;
            let row = iter.nth(2).unwrap().parse()?;
            Position { col, row }
        };

        let beacon = {
            let col = iter.nth(6).unwrap().parse()?;
            let row = iter.nth(2).unwrap().parse()?;
            Position { col, row }
        };
        Ok(Measurement { sensor, beacon })
    }
}

fn manhattan_dist(a: &Position, b: &Position) -> i32 {
    (a.row - b.row).abs() + (a.col - b.col).abs()
}

fn impossible_beacon_positions(measurements: &Vec<Measurement>, row: i32) -> Range<i32> {
    let mut ranges = Vec::new();

    for m in measurements.iter() {
        let distance_to_beacon = manhattan_dist(&m.sensor, &m.beacon);

        let distance_to_row = (m.sensor.row - row).abs();

        if distance_to_row <= distance_to_beacon {
            let dleft = distance_to_beacon - distance_to_row;
            let left = m.sensor.col - dleft;
            let right = m.sensor.col + dleft;
            ranges.push(left..(right + 1));
        }
    }

    ranges.sort_by_key(|r| (r.start, r.end));

    let mut merged_range: Range<i32> = ranges[0].clone();
    for r in ranges.iter().skip(1) {
        if r.start <= merged_range.end {
            merged_range.end = max(r.end, merged_range.end);
        }
    }
    merged_range
}

fn main() -> Result<()> {
    let measurements = aoc22::read_one_per_line::<Measurement>("./data/15.input")?;

    let target_y = 2000000;
    let positions = impossible_beacon_positions(&measurements, target_y);
    let mut answer: i32 = positions.end - positions.start;

    let mut beacons_in_target_row = HashSet::new();
    for measurement in measurements.iter() {
        if measurement.beacon.row == target_y {
            beacons_in_target_row.insert(measurement.beacon.col);
        }
    }
    answer -= beacons_in_target_row.len() as i32;
    println!("Positions in row {target_y} that can not contain a beacon: {answer}");

    let max_pos = 4000000;
    for target_y in 0..=max_pos {
        let positions = impossible_beacon_positions(&measurements, target_y);
        if positions.end <= max_pos {
            println!(
                "Beacon tuning frequency: {}",
                (positions.end as i64) * (4000000 as i64) + (target_y as i64)
            );
            break;
        }
    }
    Ok(())
}
