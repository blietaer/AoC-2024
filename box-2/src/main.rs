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

fn test_vec(list: &Vec<i32>) -> i32 {
    let list_size: usize = list.len();
    let mut prev_dir = 0;
    let mut bad_points: i32 = -1;
    for i in 1..list_size {
        let test = list[i] - list[i - 1];
        let mut new_dir = 0;

        if test == 0 {
            new_dir = 0;
        } else if test > 0 {
            new_dir = 1;
        } else if test < 0 {
            new_dir = -1;
        }
        if test == 0 {
            print!("<Same {} & {} >", list[i], list[i - 1]);
            bad_points = i as i32 - 1;
            break;
        }

        if test.abs() > 3 {
            print!("<Too steep {} & {} >", list[i], list[i - 1]);
            bad_points = i as i32 - 1;
            break;
        }

        if (new_dir != prev_dir) && (prev_dir != 0) {
            print!("<Chg.Dir. {} & {} >", list[i], list[i - 1]);
            bad_points = i as i32 - 1;
            break;
        }
        prev_dir = new_dir;
    }
    return bad_points;
}

fn is_it_safe(i: usize, list_s: &str) -> bool {
    let mut list: Vec<i32> = vec![];
    for elem in list_s.trim().split_whitespace() {
        list.push(elem.parse().expect("not an int ! "));
    }

    print!("[{}] ", i);
    for c in &list {
        print!("{},", c);
    }

    let res = test_vec(&list);
    if res >= 0 {
        let mut new_list = list.to_vec();
        print!("Removing element {}", res);
        new_list.remove(res as usize);
        let mut new_res = test_vec(&new_list);

        if new_res >= 0 && res > 0 {
            let mut new_list = list.to_vec();
            print!("Removing element {}", res - 1);
            new_list.remove(res as usize - 1);
            new_res = test_vec(&new_list);
        }

        if new_res >= 0 && res <= list.len() as i32 - 1 {
            let mut new_list = list.to_vec();
            print!("Removing element {}", res + 1);
            new_list.remove(res as usize + 1);
            new_res = test_vec(&new_list);
        }
        if new_res >= 0 {
            println!(" => BAD");
            return false;
        }
    }
    println!(" => OK");
    return true;
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

    let nbr_size: usize = report.len();
    let mut total: u64 = 0;
    for i in 0..nbr_size {
        if is_it_safe(i, &report[i]) {
            total = total + 1;
        }
    }
    println!("Total is: {}", total);
}
