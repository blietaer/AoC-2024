use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Read files
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Read
fn check_xmas(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i < 140 - 1 && j < 140 - 1 && i >= 1 && j >= 1 {
        // if i <= 139 - 1 && j <= 139 - 1 && i >= 1 && j >= 1 {
        if grid[i][j] == 'A'
            && ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
                || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
            && ((grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
                || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M'))
        {
            println!("@[{}][{}]: We have XMAS", i, j);
            return true;
        }
    }
    return false;
}

fn main() {
    let width = 140;
    let height = 140;

    let mut grid = vec![vec![' '; width]; height];
    let mut i: usize = 0;
    let mut j: usize = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            // println!("[{}] line: {}", i, line);
            // print!("[{}] ", i);
            for elem in line.chars() {
                grid[i][j] = elem;
                // println!("[{}] elem: {}", j, elem);
                // println!(" @[{}][{}]: '{}'", i, j, grid[i][j]);
                j = j + 1;
            }
            i = i + 1;
            j = 0;
        }
    } else {
        println!("File not found");
    }

    let mut total = 0;
    for i in 0..140 {
        for j in 0..140 {
            // println!("@[{}][{}]: '{}'", i, j, grid[i][j]);
            if grid[i][j] == 'A' {
                // println!("We have 'A' @[{}][{}]: checking for horiz XMASS", i, j);
                if check_xmas(i, j, &grid) {
                    total = total + 1;
                }
            }
        }
    }

    println!("Total is: {}", total);
}
