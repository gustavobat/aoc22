use anyhow::Result;

fn priority(c : char) -> u32 {
    match c {
        x if x.is_lowercase() => 1 + x as u32 - 'a' as u32,
        x if x.is_uppercase() => 27 + x as u32 - 'A' as u32,
        _ => 0
    }
}

fn priority_part_1(path: &str) -> Result<u32> {
    let lines : Vec<String> = std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .collect();
    
    let mut priority_sum : u32 = 0;
    for line in &lines {
        let container1 : &str = &line[0..line.len()/2];
        let container2 : &str = &line[line.len()/2..line.len()];

        for item in container1.chars() {
            if container2.contains(item) {
                priority_sum += priority(item);
                break;
            }
        }
    }

    Ok(priority_sum)
}

fn priority_part_2(path: &str) -> Result<u32> {
    let lines : Vec<String> = std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .collect();
    
    let mut priority_sum : u32 = 0;
    for i in 0..lines.len() / 3 {
        let rucksack1 : &str = &lines[3 * i + 0];
        let rucksack2 : &str = &lines[3 * i + 1];
        let rucksack3 : &str = &lines[3 * i + 2];

        for item in rucksack1.chars() {
            if rucksack2.contains(item) {
                if rucksack3.contains(item) {
                    priority_sum += priority(item);
                    break;
                }
            }
        }
    }

    Ok(priority_sum)
}

fn main() -> Result<()> {
    
    println!("Priority part 1: {}", priority_part_1("./data/03.input").unwrap());
    println!("Priority part 2: {}", priority_part_2("./data/03.input").unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn priority_test() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(priority_part_1("./data/03.test").unwrap(), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(priority_part_2("./data/03.test").unwrap(), 70);
    }
}
