use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read files
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Read files
fn parse_line(line: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    // let caps = re.captures(line);
    if let Some(caps) = re.captures(line) {
        let mul = caps[1].parse::<u64>().unwrap() * caps[2].parse::<u64>().unwrap();
        println!("=> {} * {} = {}", &caps[1], &caps[2], mul);
        return mul;
    } else {
        println!("=> Nothing ");
        return 0;
    }
    line.len().try_into().unwrap()
}

fn main() {
    let mut report: Vec<String> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            report.push(line)
        }
        println!("Report Size: {}", report.len());
    } else {
        println!("File not found");
    }

    let mut total: u64 = 0;
    for line in report {
        println!("{}", line);
        total = total + parse_line(&line);
    }
    println!("Total is: {}", total);
}
