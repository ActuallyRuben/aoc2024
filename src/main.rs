use std::time::Duration;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod macros;
pub(crate) mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 28.67µs, stddev 4.006µs in 17440 runs)
// day1::part2 (avg 47.358µs, stddev 5.732µs in 10558 runs)
// day2::part1 (avg 41.476µs, stddev 4.710µs in 12055 runs)
// day2::part2 (avg 91.155µs, stddev 8.500µs in 5486 runs)
// day3::part1 (avg 18.007µs, stddev 2.624µs in 27768 runs)
// day3::part2 (avg 13.535µs, stddev 2.304µs in 36940 runs)
// day4::part1 (avg 298.833µs, stddev 18.793µs in 1674 runs)
// day4::part2 (avg 65.213µs, stddev 7.674µs in 7668 runs)

fn main() {
    // puzzle!(day1: part1 = 1579939, part2 = 20351745);
    // puzzle!(day2: part1 = 483, part2 = 528);
    // puzzle!(day3: part1 = 153469856, part2 = 77055967);
    // puzzle!(day4: part1 = 2297, part2 = 1745);
    // puzzle!(day5: part1 = 5713, part2 = 5180);
    puzzle!(day6: part1 = 4890, part2 = 1995);
}
