use std::time::Duration;

mod day1;
mod day2;
mod macros;
mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 34.347µs, stddev 5.150µs in 14558 runs)
// day1::part2 (avg 52.756µs, stddev 5.080µs in 9478 runs)
// day2::part1 (avg 119.374µs, stddev 10.131µs in 4189 runs)
// day2::part2 (avg 162.896µs, stddev 12.052µs in 3070 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
}
