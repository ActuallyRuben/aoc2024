use std::time::Duration;

mod day1;
mod day2;
mod day3;
mod day4;
mod macros;
mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 31.804µs, stddev 1.534µs in 15722 runs)
// day1::part2 (avg 42.446µs, stddev 1.471µs in 11780 runs)
// day2::part1 (avg 40.395µs, stddev 0.994µs in 12378 runs)
// day2::part2 (avg 87.691µs, stddev 1.873µs in 5702 runs)
// day3::part1 (avg 15.481µs, stddev 0.598µs in 32296 runs)
// day3::part2 (avg 14.575µs, stddev 0.494µs in 34304 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
    puzzle!(day3: part1 = 153469856, part2 = 77055967);
    puzzle!(day4: part1 = 2297, part2 = 1745);
}
