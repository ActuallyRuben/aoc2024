use std::time::Duration;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod macros;
pub(crate) mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 28.224µs, stddev 5.693µs in 17715 runs)
// day1::part2 (avg 49.585µs, stddev 8.400µs in 10084 runs)
// day2::part1 (avg 35.613µs, stddev 5.832µs in 14040 runs)
// day2::part2 (avg 95.215µs, stddev 11.583µs in 5252 runs)
// day3::part1 (avg 18.423µs, stddev 4.245µs in 27140 runs)
// day3::part2 (avg 14.28µs, stddev 3.159µs in 35014 runs)
// day4::part1 (avg 269.207µs, stddev 19.201µs in 1858 runs)
// day4::part2 (avg 64.2µs, stddev 8.819µs in 7789 runs)
// day5::part1 (avg 235.647µs, stddev 18.771µs in 2122 runs)
// day5::part2 (avg 384.742µs, stddev 26.830µs in 1300 runs)
// day6::part1 (avg 20.522µs, stddev 3.833µs in 24363 runs)
// day6::part2 (avg 72.956339ms, stddev 766.387µs in 7 runs)
// day7::part1 (avg 190.264µs, stddev 19.657µs in 2628 runs)
// day7::part2 (avg 190.78µs, stddev 23.288µs in 2621 runs)
// day8::part1 (avg 27.366µs, stddev 3.319µs in 18271 runs)
// day8::part2 (avg 65.116µs, stddev 6.435µs in 7679 runs)
// day9::part1 (avg 21.274µs, stddev 2.828µs in 23503 runs)
// day9::part2 (avg 600.123967ms, stddev 0.000µs in 1 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
    puzzle!(day3: part1 = 153469856, part2 = 77055967);
    puzzle!(day4: part1 = 2297, part2 = 1745);
    puzzle!(day5: part1 = 5713, part2 = 5180);
    puzzle!(day6: part1 = 4890, part2 = 1995);
    puzzle!(day7: part1 = 3312271365652, part2 = 509463489296712);
    puzzle!(day8: part1 = 244, part2 = 912);
    puzzle!(day9: part1 = 6435922584968, part2 = 6469636832766);
}
