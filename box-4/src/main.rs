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

// Read Horiz Forward
fn check_horiz(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if j <= 139 - 3 {
        if grid[i][j] == 'X'
            && grid[i][j + 1] == 'M'
            && grid[i][j + 2] == 'A'
            && grid[i][j + 3] == 'S'
        {
            println!("@[{}][{}]: We have horiz XMAS", i, j);
            return true;
        }
    }
    return false;
}

// Read Horiz backward
fn check_horiz_back(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if j >= 3 {
        if grid[i][j] == 'X'
            && grid[i][j - 1] == 'M'
            && grid[i][j - 2] == 'A'
            && grid[i][j - 3] == 'S'
        {
            println!("@[{}][{}]: We have horiz back XMAS", i, j);
            return true;
        }
    }
    return false;
}

// Read Vert Down
fn check_vert_down(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i <= 139 - 3 {
        if grid[i][j] == 'X'
            && grid[i + 1][j] == 'M'
            && grid[i + 2][j] == 'A'
            && grid[i + 3][j] == 'S'
        {
            println!("@[{}][{}]: We have vert down XMAS", i, j);
            return true;
        }
    }
    return false;
}

// Read Vert up
fn check_vert_up(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i >= 3 {
        if grid[i][j] == 'X'
            && grid[i - 1][j] == 'M'
            && grid[i - 2][j] == 'A'
            && grid[i - 3][j] == 'S'
        {
            println!("@[{}][{}]: We have vert up XMAS", i, j);
            return true;
        }
    }
    return false;
}
// Read diag Up right
fn check_diag_up_right(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if j <= 139 - 3 && i >= 3 {
        if grid[i][j] == 'X'
            && grid[i - 1][j + 1] == 'M'
            && grid[i - 2][j + 2] == 'A'
            && grid[i - 3][j + 3] == 'S'
        {
            println!("@[{}][{}]: We have diag up right XMAS", i, j);
            return true;
        }
    }
    return false;
}
// Read diag Up Left
fn check_diag_up_left(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i >= 3 && j >= 3 {
        if grid[i][j] == 'X'
            && grid[i - 1][j - 1] == 'M'
            && grid[i - 2][j - 2] == 'A'
            && grid[i - 3][j - 3] == 'S'
        {
            println!("@[{}][{}]: We have diag up left XMAS", i, j);
            return true;
        }
    }
    return false;
}
// Read diag down Left
fn check_diag_down_left(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i <= 139 - 3 && j >= 3 {
        if grid[i][j] == 'X'
            && grid[i + 1][j - 1] == 'M'
            && grid[i + 2][j - 2] == 'A'
            && grid[i + 3][j - 3] == 'S'
        {
            println!("@[{}][{}]: We have diag down left XMAS", i, j);
            return true;
        }
    }
    return false;
}

// Read diag down Right
fn check_diag_down_right(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i <= 139 - 3 && j <= 139 - 3 {
        if grid[i][j] == 'X'
            && grid[i + 1][j + 1] == 'M'
            && grid[i + 2][j + 2] == 'A'
            && grid[i + 3][j + 3] == 'S'
        {
            println!("@[{}][{}]: We have diag down right XMAS", i, j);
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
            if grid[i][j] == 'X' {
                // println!("We have 'X' @[{}][{}]: checking for horiz XMASS", i, j);
                if check_horiz(i, j, &grid) {
                    total = total + 1;
                }
                if check_horiz_back(i, j, &grid) {
                    total = total + 1;
                }
                if check_vert_down(i, j, &grid) {
                    total = total + 1;
                }
                if check_vert_up(i, j, &grid) {
                    total = total + 1;
                }
                if check_diag_up_right(i, j, &grid) {
                    total = total + 1;
                }
                if check_diag_up_left(i, j, &grid) {
                    total = total + 1;
                }
                if check_diag_down_left(i, j, &grid) {
                    total = total + 1;
                }
                if check_diag_down_right(i, j, &grid) {
                    total = total + 1;
                }
            }
        }
    }

    println!("Total is: {}", total);
}
