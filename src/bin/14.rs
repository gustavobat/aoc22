use anyhow::Result;

fn make_grid(paths: &mut Vec<Vec<(usize, usize)>>, has_floor: bool) -> (Vec<Vec<char>>, usize) {
    let mut max_x = 0;
    let mut min_x = 1000;
    let mut max_y = 0;
    let mut min_y = 1000;

    let mut grid: Vec<Vec<char>> = Vec::new();
    for path in paths.iter() {
        for (_, y) in path {
            if *y > max_y { max_y = *y; }
        }
    }

    if has_floor {
        paths.push(vec![(320, max_y + 2), (680, max_y + 2)]);
        max_y += 2;
    };

    for path in paths.iter() {
        for (x, y) in path {
            if *x < min_x { min_x = *x; }
            if *x > max_x { max_x = *x; }
            if *y < min_y { min_y = *y; }
        }
    }

    grid.resize(max_y + 1, Vec::<char>::new());
    for row in &mut grid {
        row.resize(max_x - min_x + 1, '.');
    }

    for path in paths {
        for line in path.windows(2) {
            let (start_x, start_y) = line[0];
            let (end_x, end_y) = line[1];
            if start_x == end_x {
                let range = if start_y < end_y {
                    start_y..=end_y
                } else {
                    end_y..=start_y
                };
                for y in range {
                    grid[y][start_x - min_x] = '#';
                }
            }
            if start_y == end_y {
                let range = if start_x < end_x {
                    start_x..=end_x
                } else {
                    end_x..=start_x
                };
                for x in range {
                    grid[start_y][x - min_x] = '#';
                }
            }
        }
    }
    let source_x = 500 - min_x;
    (grid, source_x)
}

fn solve(grid: &mut Vec<Vec<char>>, source_x: usize) -> usize {
    let mut sand_units_counter = 0;
    let mut finished = false;
    while !finished {
        let (mut cur_x, mut cur_y) = (source_x, 0);
        let mut can_move = true;
        while can_move {
            can_move = false;
            let dx_arr = [0, -1, 1];
            for dx in dx_arr {
                if (cur_x == 0 && dx == -1) || cur_y == grid.len() - 1 {
                    finished = true;
                    break;
                }
                let (next_x, next_y) = ((cur_x as i32 + dx) as usize, cur_y + 1);
                if grid[next_y][next_x] == '.' {
                    cur_x = next_x;
                    cur_y = next_y;
                    can_move = true;
                    break;
                }
            }
            if finished {
                break;
            }
            if !can_move {
                grid[cur_y][cur_x] = 'o';
                sand_units_counter += 1;
                if (cur_x, cur_y) == (source_x, 0) {
                    finished = true;
                }
            };
        }
    }

    sand_units_counter
}

fn main() -> Result<()> {
    let mut paths: Vec<_> = std::fs::read_to_string("./data/14.input")?
        .lines()
        .map(|line| {
            line.split(" -> ")
                .collect::<Vec<_>>()
                .iter()
                .map(|pos_str| {
                    let split = pos_str.split_once(",").unwrap();
                    (
                        split.0.parse::<usize>().unwrap(),
                        split.1.parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let (mut grid_with_abyss, source_x) = make_grid(&mut paths, false);
    let res = solve(&mut grid_with_abyss, source_x);
    println!("Units of sand that come to rest before falling into the abyss: {res}");

    let (mut grid_with_floor, source_x) = make_grid(&mut paths, true);
    let res = solve(&mut grid_with_floor, source_x);
    println!("Units of sand that come to rest before blocking the source: {res}");

    Ok(())
}
