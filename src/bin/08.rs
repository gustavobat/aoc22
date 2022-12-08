use anyhow::Result;

fn is_visible(i : usize, j : usize, grid: &Vec<Vec<u32>>) -> bool {
    
    let rows = grid.len();
    let cols = grid[0].len();

    // Handle edge trees
    if i == 0 || j == 0 {
        return true;
    }
    if i == rows - 1|| j == cols - 1 {
        return true;
    }

    // Current height
    let cur_height = grid[i][j];

    macro_rules! check_if_is_visible_from_path {
        ($range: expr, $horizontal: expr) => {
            let mut is_visible = true;
            for k in $range {
                let height = match $horizontal {
                    true => grid[i][k],
                    false => grid[k][j],
                };
                if height >= cur_height {
                    is_visible = false;
                }
            }
            if is_visible {
                return true;
            }
        };
    }
    
    let horizontal = true;
    let vertical = false;

    check_if_is_visible_from_path!(0..j, horizontal);
    check_if_is_visible_from_path!((j + 1..cols).rev(), horizontal);
    check_if_is_visible_from_path!(0..i, vertical);
    check_if_is_visible_from_path!((i + 1..rows).rev(), vertical);

    false
}

fn scenic_score(i : usize, j : usize, grid: &Vec<Vec<u32>>) -> u32 {
    
    let rows = grid.len();
    let cols = grid[0].len();

    // Handle edge trees
    if i == 0 || j == 0 {
        return 0;
    }
    if i == rows - 1|| j == cols - 1 {
        return 0;
    }

    // Current height
    let cur_height = grid[i][j];
    
    let mut scenic_score = 1;
    macro_rules! scenic_score_from_path {
        ($range: expr, $horizontal: expr) => {
            let mut visible_trees = 0;
            for k in $range {
                visible_trees += 1;
                let height = match $horizontal {
                    true => grid[i][k],
                    false => grid[k][j],
                };
                if height >= cur_height {
                    break;
                }
            }
            scenic_score *= visible_trees;
        };
    }
    
    let horizontal = true;
    let vertical = false;

    scenic_score_from_path!((0..j).rev(), horizontal);
    scenic_score_from_path!(j + 1..cols, horizontal);
    scenic_score_from_path!((0..i).rev(), vertical);
    scenic_score_from_path!(i + 1..rows, vertical);
    
    scenic_score
}

fn main () -> Result<()>{
    let grid = aoc22::read_one_per_line::<String>("./data/08.input")?
        .iter()
        .map(|row|
             row
             .chars()
             .map(|c| c.to_digit(10).expect("Could not parse digit to u32"))
             .collect::<Vec<u32>>())
             .collect::<Vec<Vec<u32>>>();

    let mut n_visible : usize = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(i, j, &grid) {
                n_visible += 1;
            }
        }
    }
    println!("Number of visible trees: {n_visible}");

    let mut highest_scenic_score : u32 = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let cur_scenic_score = scenic_score(i, j, &grid);
            if cur_scenic_score > highest_scenic_score {
                highest_scenic_score = cur_scenic_score;
            }
        }
    }
    println!("Highest scenic score: {highest_scenic_score}");


    Ok(())
}
