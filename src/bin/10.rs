use anyhow::Result;
use aoc22::read_one_per_line;

fn get_acc_signal_strength(commands : &Vec<String>) -> i32 {
    // Init variables
    let mut x_reg = 1;
    let mut clock_cycles = 0;
    let mut acc_signal_strength = 0;

    // Clock tick callback
    let mut on_tick = |x_reg : &mut i32| {
        if (clock_cycles + 1) % 40 == 20 {
            acc_signal_strength += *x_reg * (clock_cycles + 1);
        }
        clock_cycles += 1;
    };

    // Process commands
    for cmd in commands {
        on_tick(&mut x_reg);
        if cmd != "noop" {
            let (_, val_str) = cmd.split_once(" ").unwrap();
            let val = val_str.parse::<i32>().unwrap();
            on_tick(&mut x_reg);
            x_reg += val;
        }
    }
    acc_signal_strength
}

fn draw_display_image(commands : &Vec<String>) -> String {
    // Init variables
    let mut x_reg = 1;
    let mut clock_cycles = 0;
    let mut display = String::new();

    // Clock tick callback
    let mut on_tick = |x_reg : &mut i32| {
        let pos = clock_cycles % 40;
        if pos == 0 && clock_cycles != 0 {
            display += "\n";
        }
        let sprite_pos : [i32; 3] = [*x_reg - 1, *x_reg, *x_reg + 1];
        if sprite_pos.contains(&pos) { display += "#"; } else { display += " "; }
        clock_cycles += 1;
    };

    // Process commands
    for cmd in commands {
        on_tick(&mut x_reg);
        if cmd != "noop" {
            let (_, val_str) = cmd.split_once(" ").unwrap();
            let val = val_str.parse::<i32>().unwrap();
            on_tick(&mut x_reg);
            x_reg += val;
        }
    }
    display
}

fn main () -> Result<()> {
    let commands = read_one_per_line::<String>("./data/10.input")?;

    println!("Accumulated signal strength: {}", get_acc_signal_strength(&commands));
    println!("Display image:\n{}", draw_display_image(&commands));

    Ok(())
}
