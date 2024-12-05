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

fn main() {
    let mut report: Vec<String> = vec![];

    if let Ok(lines) = read_lines("input_a.txt") {
        for line in lines.flatten() {
            let parts = line.split('|');
            if parts.count() != 2 {
                dbg!("FUUUCk ! ");
            }
        }
        println!("Report Size: {}", report.len());
    } else {
        println!("File not found");
    }

    // let nbr_size: usize = report.len();
    // let mut total: u64 = 0;
    // for i in 0..nbr_size {
    //     if is_it_safe(i, &report[i]) {
    //         total = total + 1;
    //     }
    // }
    // println!("Total is: {}", total);
}
