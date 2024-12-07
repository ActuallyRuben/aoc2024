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
// day1::part1 (avg 28.238µs, stddev 3.894µs in 17707 runs)
// day1::part2 (avg 50.064µs, stddev 4.525µs in 9988 runs)
// day2::part1 (avg 38.397µs, stddev 3.446µs in 13022 runs)
// day2::part2 (avg 94.818µs, stddev 10.448µs in 5274 runs)
// day3::part1 (avg 17.669µs, stddev 1.782µs in 28297 runs)
// day3::part2 (avg 14.261µs, stddev 1.779µs in 35059 runs)
// day4::part1 (avg 260.76µs, stddev 13.461µs in 1918 runs)
// day4::part2 (avg 58.523µs, stddev 5.781µs in 8544 runs)
// day5::part1 (avg 237.363µs, stddev 10.292µs in 2107 runs)
// day5::part2 (avg 382.706µs, stddev 13.036µs in 1307 runs)
// day6::part1 (avg 21.128µs, stddev 2.412µs in 23666 runs)
// day6::part2 (avg 72.430939ms, stddev 628.537µs in 7 runs)
// day7::part1 (avg 195.52µs, stddev 13.540µs in 2558 runs)
// day7::part2 (avg 196.256µs, stddev 21.519µs in 2548 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
    puzzle!(day3: part1 = 153469856, part2 = 77055967);
    puzzle!(day4: part1 = 2297, part2 = 1745);
    puzzle!(day5: part1 = 5713, part2 = 5180);
    puzzle!(day6: part1 = 4890, part2 = 1995);
    puzzle!(day7: part1 = 3312271365652, part2 = 509463489296712);
}
