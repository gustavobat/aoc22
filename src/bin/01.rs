use anyhow::Result;

fn sum_first_n_elements(vec: &Vec<u32>, n: usize) -> u32 {
    let mut sum : u32 = 0;
    for i in 0..n {
        sum += vec[i];
    }
    sum
}

fn main() -> Result<()> {
    let mut calories_per_elf = std::fs::read_to_string("./data/01.input")?
        .split("\n\n")
        .map(|s| s
             .split_whitespace()
             .map(|n| match n.parse::<u32>() {
                 Ok(n) => n,
                 Err(_) => panic!("Error parsing {:?} to u32", n),
             })
             .sum()
            )
        .collect::<Vec<u32>>();

    calories_per_elf.sort_by(|a, b| b.cmp(a));

    println!("Max: {:?}", sum_first_n_elements(&calories_per_elf, 1));
    println!("Max three: {:?}", sum_first_n_elements(&calories_per_elf, 3));

    Ok(())
}
