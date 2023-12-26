use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day24/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total: {}", solution(&contents))
}

#[derive(Debug)]
struct Line {
    x0: i64,
    y0: i64,
    z0: i64,

    vx: i64,
    vy: i64,
    vz: i64,
}

fn calculate_slop(vy: i64, vx: i64) -> f64 {
    vy as f64 / vx as f64
}

fn is_two_lines_intersects_in_an_interval(a: &Line, b: &Line) -> bool {
    const START_INTERVAL: u64 = 200000000000000;
    const END_INTERVAL: u64 = 400000000000000;

    let m1 = calculate_slop(a.vy, a.vx);
    let m2 = calculate_slop(b.vy, b.vx);

    if m1 == m2 {
        return false;
    }

    let x = (m1 * a.x0 as f64 - m2 * b.x0 as f64 + b.y0 as f64 - a.y0 as f64) / (m1 - m2);
    // println!("    x: {}, y: {}", x, m1 * (x - a.x0 as f64) + a.y0 as f64);
    if a.vx < 0 {
        if x > a.x0 as f64 {
            return false;
        }
    }
    if a.vx > 0 {
        if x < a.x0 as f64 {
            return false;
        }
    }
    if b.vx < 0 {
        if x > b.x0 as f64 {
            return false;
        }
    }
    if b.vx > 0 {
        if x < b.x0 as f64 {
            return false;
        }
    }

    if START_INTERVAL as f64 <= x && x <= END_INTERVAL as f64 {
        let y = m1 * (x - a.x0 as f64) + a.y0 as f64;
        // if a.vy < 0 {
        //     if y > a.y0 as f64 {
        //         return false;
        //     }
        // }
        // if a.vy > 0 {
        //     if y < a.y0 as f64 {
        //         return false;
        //     }
        // }
        return START_INTERVAL as f64 <= y && y <= END_INTERVAL as f64;
    }
    false
}

fn parse_data(line: &str) -> Line {
    let line = line
        .split(" ")
        .filter_map(|x| {
            if x != "@" && x != "" {
                x.replace(",", "").parse::<i64>().ok()
            } else {
                None
            }
        })
        .collect::<Vec<i64>>();

    Line {
        x0: line[0],
        y0: line[1],
        z0: line[2],

        vx: line[3],
        vy: line[4],
        vz: line[5],
    }
}

fn solution(input: &str) -> usize {
    let mut lines: Vec<Line> = vec![];
    for line in input.lines() {
        lines.push(parse_data(line));
    }

    let mut intersect_count = 0;

    while lines.len() > 1 {
        let line_a = lines.remove(0);

        for line_b in lines.iter() {
            if is_two_lines_intersects_in_an_interval(&line_a, &line_b) {
                intersect_count += 1;
            }
        }
    }

    intersect_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_data() {
        let line = "19, 13, 30 @ -2,  1, -2";

        let linear_line = parse_data(line);
        assert_eq!(linear_line.x0, 19);
        assert_eq!(linear_line.y0, 13);
        assert_eq!(linear_line.z0, 30);
        assert_eq!(linear_line.vx, -2);
        assert_eq!(linear_line.vy, 1);
        assert_eq!(linear_line.vz, -2);

        let line = "20, 19, 15 @  1, -5, -3";

        let linear_line = parse_data(line);
        assert_eq!(linear_line.x0, 20);
        assert_eq!(linear_line.y0, 19);
        assert_eq!(linear_line.z0, 15);
        assert_eq!(linear_line.vx, 1);
        assert_eq!(linear_line.vy, -5);
        assert_eq!(linear_line.vz, -3);
    }

    #[test]
    fn it_can_calculate_slop() {
        let m = calculate_slop(1, 2);
        assert_eq!(m, 0.5);

        let m = calculate_slop(2, 1);
        assert_eq!(m, 2.0);

        let m = calculate_slop(3, 1);
        assert_eq!(m, 3.0);
    }

    #[test]
    fn it_can_check_two_lines_intersects_in_an_interval() {
        let a = Line {
            x0: 19,
            y0: 13,
            z0: 30,

            vx: -2,
            vy: 1,
            vz: -2,
        };

        let b = Line {
            x0: 20,
            y0: 19,
            z0: 15,

            vx: 1,
            vy: -5,
            vz: -3,
        };

        assert_eq!(is_two_lines_intersects_in_an_interval(&a, &b), true);

        let a = Line {
            x0: 19,
            y0: 13,
            z0: 30,

            vx: -2,
            vy: 1,
            vz: -2,
        };

        let b = Line {
            x0: 12,
            y0: 31,
            z0: 28,

            vx: -1,
            vy: -2,
            vz: -1,
        };

        assert_eq!(is_two_lines_intersects_in_an_interval(&a, &b), false);

        let a = Line {
            x0: 18,
            y0: 19,
            z0: 22,

            vx: -1,
            vy: -1,
            vz: -2,
        };

        let b = Line {
            x0: 20,
            y0: 25,
            z0: 34,

            vx: -2,
            vy: -2,
            vz: -4,
        };

        assert_eq!(is_two_lines_intersects_in_an_interval(&a, &b), false);
    }
}
