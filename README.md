# Advent Of Code 2023

## Testing

### Test everything

```bash
cargo test --workspace
```

### Test a crate or lib

```bash
cargo test -p day06
```

or

```bash
cargo test -p day14
```

###

## Run solution

Change the code inside `main.or` to each solution.

### Run day 13 part 1

```rust
use day13::part2;

fn main() {
    println!("{}", part2::solution::main());
}
```

### Run day 14 part 1

```rust
use day14::part1;

fn main() {
    println!("{}", part1::solution::main());
}
```
