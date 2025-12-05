use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_query, file_path) = parse_config(&args);

    println!("In file {file_path}");

    if let Ok(lines) = read_lines(file_path) {
        let result = solve(lines);
        println!("Result: {}", result);
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn solve(lines: io::Lines<io::BufReader<File>>) -> u32 {

    let mut position = 50;
    let mut count_zeros = 0;

    for line in lines.map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (operator_str, rest) = line.split_at(1);
        let operator = operator_str.chars().next().unwrap();

        if operator != 'L' && operator != 'R' {
            continue;
        }
        let mut number_on_dial:u32 = match rest.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        number_on_dial = number_on_dial % 100;

        if operator == 'L' {
            position = (position + 100 - number_on_dial) % 100;
        } else {
           position = (position + number_on_dial) % 100;
        }

        if position == 0 {
            count_zeros += 1;
        }
    }

    return count_zeros;
}
