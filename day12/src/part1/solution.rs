use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day11/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

fn extract_chars_and_order(line: &str) -> (Vec<char>, Vec<u32>) {
    let mut line = line.split(" ");
    (
        line.next().unwrap().chars().collect(),
        line.next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect(),
    )
}

fn is_able_to_accommodate(chars: &[char], amount: &u32) -> Option<usize> {
    let mut optional_pending = 0;
    let mut available_pending = 0;

    let mut able_to_accommodate = false;

    let last_index = chars.len() - 1;

    // "??# 2";

    for (index, char) in chars.iter().enumerate() {
        match char {
            '.' => {
                if optional_pending == *amount || available_pending == *amount {
                    able_to_accommodate = true;
                }

                optional_pending = 0;
                available_pending = 0;
            }
            '?' => {
                if optional_pending == *amount || available_pending == *amount {
                    able_to_accommodate = true;
                }

                optional_pending += 1;
                available_pending += 1;

                if index == last_index {
                    if optional_pending == *amount {
                        able_to_accommodate = true;
                    }
                }
            }
            '#' => {
                available_pending += 1;
                optional_pending = 0;

                if index == last_index {
                    if available_pending == *amount {
                        able_to_accommodate = true;
                    }
                }
            }
            _ => {}
        }
    }

    None
}

fn solution(input: &str) -> usize {
    let mut total = 0;

    let line = "???.### 1,1,3";
    println!("{:?}", extract_chars_and_order(line));

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_can_extract_chars_and_order() {
        let line = "???.### 1,1,3";
        let (chars, order) = super::extract_chars_and_order(line);

        assert_eq!(chars, vec!['?', '?', '?', '.', '#', '#', '#']);
        assert_eq!(order, vec![1, 1, 3]);
    }
}
