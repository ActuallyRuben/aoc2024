use std::time::Duration;

mod day1;
mod day2;
mod macros;
mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 27.502µs, stddev 4.279µs in 18181 runs)
// day1::part2 (avg 46.448µs, stddev 6.685µs in 10765 runs)
// day2::part1 (avg 78.088µs, stddev 6.640µs in 6404 runs)
// day2::part2 (avg 124.095µs, stddev 13.824µs in 4030 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
}
