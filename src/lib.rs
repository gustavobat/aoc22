use anyhow::Result;
use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn read_one_every_double_linebreak<T>(path: &str) -> Result<Vec<T>>
    where
        T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split("\n\n")
        .filter_map(|str| str.parse::<T>().ok())
        .collect())
}

