use std::time::Duration;

mod day1;
mod macros;
mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
}
