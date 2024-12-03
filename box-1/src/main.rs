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

fn calculate_diff(pos: usize, val_a: i32, val_b: i32) -> i32 {
    print!("[{}] {} - {} ", pos, val_a, val_b);
    let diff: i32 = val_a - val_b;
    println!("= {} ", diff.abs());
    diff.abs()
}

fn calculate_similar(check_val: i32, right_nbr: &Vec<i32>) -> i32 {
    let right_nbr_size: usize = right_nbr.len();
    let mut total: i32 = 0;
    let mut count = 0;

    for i in 0..right_nbr_size {
        if right_nbr[i] == check_val {
            count = count + 1;
        }
    }
    total = count * check_val;
    total
}

fn main() {
    let mut left_nbr: Vec<i32> = vec![];
    let mut right_nbr: Vec<i32> = vec![];
    let mut diff: Vec<i32> = vec![];

    if let Ok(lines) = read_lines("left_sort.txt") {
        for line in lines.flatten() {
            left_nbr.push(line.trim().parse().unwrap())
        }
        println!("Left: {}", left_nbr.len());
    }

    if let Ok(lines) = read_lines("right_sort.txt") {
        for line in lines.flatten() {
            right_nbr.push(line.trim().parse().unwrap())
        }
        println!("Right: {}", right_nbr.len());
    }

    let left_nbr_size: usize = left_nbr.len();
    let mut total: u64 = 0;
    for i in 0..left_nbr_size {
        // let rest = calculate_diff(i, left_nbr[i], right_nbr[i]);
        let rest = calculate_similar(left_nbr[i], &right_nbr);
        diff.push(rest);
        total = total + rest as u64;
    }
    println!("Total is: {}", total);
}
