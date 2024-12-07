use std::time::Duration;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod macros;
pub(crate) mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 27.38µs, stddev 5.182µs in 18262 runs)
// day1::part2 (avg 47.656µs, stddev 6.457µs in 10492 runs)
// day2::part1 (avg 39.665µs, stddev 7.012µs in 12606 runs)
// day2::part2 (avg 95.666µs, stddev 8.510µs in 5227 runs)
// day3::part1 (avg 17.621µs, stddev 2.807µs in 28375 runs)
// day3::part2 (avg 14.973µs, stddev 2.564µs in 33392 runs)
// day4::part1 (avg 260.494µs, stddev 18.708µs in 1920 runs)
// day4::part2 (avg 62.414µs, stddev 8.952µs in 8011 runs)
// day5::part1 (avg 235.894µs, stddev 18.501µs in 2120 runs)
// day5::part2 (avg 376.18µs, stddev 22.669µs in 1330 runs)
// day6::part1 (avg 20.215µs, stddev 2.373µs in 24734 runs)
// day6::part2 (avg 206.07503ms, stddev 416.127µs in 3 runs)

fn main() {
    // puzzle!(day1: part1 = 1579939, part2 = 20351745);
    // puzzle!(day2: part1 = 483, part2 = 528);
    // puzzle!(day3: part1 = 153469856, part2 = 77055967);
    // puzzle!(day4: part1 = 2297, part2 = 1745);
    // puzzle!(day5: part1 = 5713, part2 = 5180);
    // puzzle!(day6: part1 = 4890, part2 = 1995);
    puzzle!(day7: part1 = 3312271365652);
}
