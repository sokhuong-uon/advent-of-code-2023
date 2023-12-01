fn main() {
    println!("Part 1: Hello, world!");
    let input = include_str!("./in.txt");
    let output = p1(input);
    dbg!(output);
}

fn p1(input: &str) -> String {
    "hi".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = p1("");
        assert_eq!(result, "hi");
    }
}
