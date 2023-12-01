use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("./src/bin/in.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", p1(&contents));
}

fn map_number(str: &str) -> &str {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => str,
    }
}

fn write_to_file(
    file: &mut File,
    line: &str,
    digit_pair: (&str, &str),
    numeric: &str,
    file_name: &str,
) {
    writeln!(file, "").unwrap();

    writeln!(file, "{line}").unwrap();
    writeln!(file, "{} {}", digit_pair.0, digit_pair.1).unwrap();
    writeln!(file, "{numeric}").unwrap();
}

fn separate_input(input: &str) {
    let ranges = [
        (1, 0, 50),
        (2, 50, 100),
        (3, 100, 150),
        (4, 150, 200),
        (5, 200, 250),
        (6, 250, 300),
        (7, 300, 350),
        (8, 350, 400),
        (9, 400, 450),
        (10, 450, 500),
        (11, 500, 550),
        (12, 550, 600),
        (13, 600, 650),
        (14, 650, 700),
        (15, 700, 750),
        (16, 750, 800),
        (17, 800, 850),
        (18, 850, 900),
        (19, 900, 950),
        (20, 950, 1000),
    ];

    for (i, (file_name, start, end)) in ranges.iter().enumerate() {
        let mut file = File::options()
            .append(true)
            .open(format!("./src/bin/{file_name}.txt"))
            .unwrap();

        for (index, line) in input.lines().enumerate() {
            if index >= *start && index < *end {
                writeln!(file, "{}", line).unwrap();
            }
        }
    }
}

fn create_file(name: &str) -> File {
    let mut file = File::create(format!("./src/bin/{name}.txt")).unwrap();
    file
}

fn p1(input: &str) -> i64 {
    // let mut file = File::options()
    //     .append(true)
    //     .open("./src/bin/out.txt")
    //     .unwrap();

    // writeln!(file, "").unwrap();
    // writeln!(file, "{file_name} ==========").unwrap();

    let mut sum = 0;

    let matches = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut digit_pair = ("", "");

        let mut left_most_index = line.len();
        let mut right_most_index: isize = -1;

        for (index, number) in matches.iter().enumerate() {
            let left_option = line.find(number);
            if left_option.is_some() {
                let i = left_option.unwrap();
                if i < left_most_index {
                    left_most_index = i;
                    digit_pair.0 = matches[index];
                }
            }

            let right_option = line.rfind(number);
            println!("right most: {:?}", right_most_index);
            if right_option.is_some() {
                let i = right_option.unwrap();
                println!("{:?}", right_option);
                println!("{:?}", i);
                if i as isize > right_most_index {
                    right_most_index = i as isize;
                    digit_pair.1 = matches[index];
                }
            }
        }

        let numeric = format!("{}{}", map_number(digit_pair.0), map_number(digit_pair.1));

        sum += numeric.parse::<i64>().unwrap();

        // write_to_file(&mut file, line, digit_pair, &numeric, file_name);
    }

    sum
}
