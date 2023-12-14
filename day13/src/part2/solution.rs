use itertools::Itertools;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day13/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

type Mirror = (usize, usize, bool);

fn find_vertical_mirror_per_line(line: &str) -> Vec<(usize, usize)> {
    let chars = line.chars();
    let mut visited_chars: Vec<char> = vec![];
    let mut mirrors: Vec<Mirror> = vec![];

    // #.##..##.
    for (index, char) in chars.enumerate() {
        if index == 0 {
            visited_chars.push(char);
            continue;
        }

        if char == visited_chars[visited_chars.len() - 1] {
            mirrors.push((index - 1, index, true));
        }

        for mirror in mirrors.iter_mut() {
            let distance = index - mirror.1;

            if distance <= mirror.0 && char != visited_chars[mirror.0 - distance] {
                mirror.2 = false;
            }
        }

        visited_chars.push(char);
    }

    mirrors
        .iter()
        .filter(|mirror| mirror.2)
        .map(|mirror| (mirror.0, mirror.1))
        .collect()
}

fn is_consecutive_lines_have_only_one_character_different(line1: &str, line2: &str) -> bool {
    let mut count = 0;
    for (char1, char2) in line1.chars().zip(line2.chars()) {
        if char1 != char2 {
            count += 1;
        }
    }

    count == 1
}

fn find_new_vertical_mirror(mirrors: Vec<Vec<(usize, usize)>>) -> Option<(usize, usize)> {
    mirrors
        .iter()
        .flatten()
        .sorted()
        .dedup_with_count()
        .find_map(|(count, mirror)| {
            if count == mirrors.len() - 1 {
                Some(mirror)
            } else {
                None
            }
        })
        .cloned()
}

fn find_new_horizontal_mirror(pattern: &str) -> Option<(usize, usize)> {
    let lines = pattern.lines();
    let mut visited_lines: Vec<&str> = vec![];
    let mut mirrors: Vec<(usize, usize, u8)> = vec![];

    // #.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.
    for (index, line) in lines.enumerate() {
        if index == 0 {
            visited_lines.push(line);
            continue;
        }

        if line == visited_lines[visited_lines.len() - 1] {
            mirrors.push((index - 1, index, 0));
        } else {
            if is_consecutive_lines_have_only_one_character_different(
                line,
                visited_lines[visited_lines.len() - 1],
            ) {
                mirrors.push((index - 1, index, 0));
            }
        }

        for mirror in mirrors.iter_mut() {
            let distance = index - mirror.1;

            if distance <= mirror.0 && line != visited_lines[mirror.0 - distance] {
                mirror.2 += 1;
            }
        }

        visited_lines.push(line);
    }

    mirrors.iter().find_map(|mirror| {
        if mirror.2 == 1 {
            Some((mirror.0, mirror.1))
        } else {
            None
        }
    })
}

fn summarize_pattern(pattern: &str) -> usize {
    let mut total = 0;

    let mirrors = pattern
        .lines()
        .map(|line| find_vertical_mirror_per_line(line))
        .collect::<Vec<Vec<(usize, usize)>>>();

    let mut vertical_mirror: Option<(usize, usize)> = None;
    let mut horizontal_mirror: Option<(usize, usize)> = None;

    if let Some(new_mirror) = find_new_vertical_mirror(mirrors) {
        vertical_mirror = Some(new_mirror);
    } else {
        if let Some(new_horizontal) = find_new_horizontal_mirror(pattern) {
            horizontal_mirror = Some(new_horizontal);
        }
    }
    if vertical_mirror.is_some() {
        total += vertical_mirror.unwrap().0 + 1;
    }
    if horizontal_mirror.is_some() {
        total += 100 * (horizontal_mirror.unwrap().0 + 1);
    }

    total
}

fn solution(input: &str) -> usize {
    let mut total = 0;

    let mut pattern = String::new();

    for line in input.lines() {
        if line.is_empty() {
            total += summarize_pattern(&pattern);
            pattern.clear();
            continue;
        }

        pattern.push_str(line);
        pattern.push('\n');
    }
    total += summarize_pattern(&pattern);

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_can_find_vertical_mirrors_per_line() {
        let line = "#.##..##.";
        let mirrors = super::find_vertical_mirror_per_line(line);
        assert_eq!(mirrors, vec![(4, 5), (6, 7)]);

        let line = "..#.##.#.";
        let mirrors = super::find_vertical_mirror_per_line(line);
        assert_eq!(mirrors, vec![(0, 1), (4, 5)]);

        let line = "##......#";
        let mirrors = super::find_vertical_mirror_per_line(line);
        assert_eq!(mirrors, vec![(0, 1), (4, 5)]);

        let line = "..#.##.#.";
        let mirrors = super::find_vertical_mirror_per_line(line);
        assert_eq!(mirrors, vec![(0, 1), (4, 5)]);

        let line = "..##..##.";
        let mirrors = super::find_vertical_mirror_per_line(line);
        assert_eq!(mirrors, vec![(0, 1), (2, 3), (4, 5), (6, 7)]);

        let line = "#.#.##.#.";
        let mirrors = super::find_vertical_mirror_per_line(line);
        assert_eq!(mirrors, vec![(4, 5)]);
    }

    #[test]
    fn it_can_find_new_vertical_mirror() {
        let mirrors = vec![
            vec![(4, 5), (6, 7)],
            vec![(0, 1), (4, 5)],
            vec![(0, 1), (4, 5)],
            vec![(0, 1), (4, 5)],
            vec![(0, 1), (2, 3), (4, 5), (6, 7)],
            vec![(0, 1), (4, 5)],
        ];

        let mirror = super::find_new_vertical_mirror(mirrors);
        assert_eq!(mirror, Some((0, 1)));

        let mirrors = vec![
            vec![(3, 5), (6, 7)],
            vec![(0, 1), (4, 5)],
            vec![(0, 1), (4, 5)],
            vec![(0, 1), (4, 5)],
            vec![(0, 1), (2, 3), (4, 5), (6, 7)],
            vec![(6, 5)],
        ];

        let mirror = super::find_new_vertical_mirror(mirrors);
        assert_eq!(mirror, None);
    }
}
