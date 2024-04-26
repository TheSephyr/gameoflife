use std::{io, u8};

use rand::Rng;

const ALIVE: &str = "O";
const DEAD: &str = ".";
const COLS: usize = 50;
const ROWS: usize = 25;

fn main() {
    print_help();
    let mut input_string = String::new();
    let mut running: bool = true;
    let mut grid = init_grid();
    while running {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        match input_string.trim().to_uppercase().as_str() {
            "Q" => {
                running = false;
                println!("False");
            }
            "N" => {
                grid = next_generation(&grid);
                print_grid(&grid)
            }
            "S" => grid = init_grid(),
            "P" => print_grid(&grid),
            "C" => print_neighbour_count(&grid),
            "H" => print_help(),
            _ => {
                println!("Command not found!");
                print_help();
            }
        }
    }
}

fn init_grid() -> Vec<Vec<&'static str>> {
    let mut grid = vec![vec![DEAD; ROWS]; COLS];
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut rng = rand::thread_rng();
            let alive = rng.gen_bool(0.5);
            if alive {
                grid[j][i] = ALIVE;
            }
        }
    }
    return grid;
}

fn print_grid(grid: &Vec<Vec<&str>>) {
    for i in 0..ROWS {
        for j in 0..COLS {
            print!("{} ", grid[j][i])
        }
        println!();
    }
}

fn print_neighbour_count(grid: &Vec<Vec<&str>>) {
    for i in 0..ROWS {
        for j in 0..COLS {
            print!("{} ", count_neighbours(grid, j, i))
        }
        println!();
    }
}

fn print_help() {
    println!("[S]tart new game");
    println!("[N]ext generation");
    println!("[H]elp");
    println!("[P]rint grid");
    println!("[C]ount neighbours");
    println!("[Q]uit");
}

fn next_generation(grid: &Vec<Vec<&str>>) -> Vec<Vec<&'static str>> {
    let mut updated_grid = vec![vec![DEAD; ROWS]; COLS];
    for i in 0..ROWS {
        for j in 0..COLS {
            let neighbour_count = count_neighbours(grid, j, i);
            if grid[j][i] == DEAD {
                if neighbour_count == 3 {
                    updated_grid[j][i] = ALIVE;
                }
            } else {
                if neighbour_count < 2 || neighbour_count > 3 {
                    updated_grid[j][i] = DEAD;
                } else {
                    updated_grid[j][i] = ALIVE;
                }
            }
        }
    }
    return updated_grid;
}

fn count_neighbours(grid: &Vec<Vec<&str>>, row_number: usize, col_number: usize) -> u8 {
    let mut count: u8 = 0;
    for i in 0..=2 {
        for j in 0..=2 {
            if !(i == 1 && j == 1) {
                let current_row = row_number + i;
                let current_col = col_number + j;
                if current_row >= 1
                    && current_col >= 1
                    && current_row <= COLS
                    && current_col <= ROWS
                {
                    if grid[current_row - 1][current_col - 1] == ALIVE {
                        count = count + 1;
                    }
                }
            }
        }
    }
    return count;
}
