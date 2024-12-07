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
// day1::part1 (avg 28.588µs, stddev 5.429µs in 17490 runs)
// day1::part2 (avg 51.526µs, stddev 7.949µs in 9705 runs)
// day2::part1 (avg 39.001µs, stddev 7.331µs in 12820 runs)
// day2::part2 (avg 97.931µs, stddev 11.958µs in 5106 runs)
// day3::part1 (avg 17.983µs, stddev 3.788µs in 27804 runs)
// day3::part2 (avg 13.462µs, stddev 3.605µs in 37142 runs)
// day4::part1 (avg 265.715µs, stddev 14.518µs in 1882 runs)
// day4::part2 (avg 58.644µs, stddev 5.670µs in 8526 runs)
// day5::part1 (avg 233.986µs, stddev 9.923µs in 2137 runs)
// day5::part2 (avg 375.288µs, stddev 15.095µs in 1333 runs)
// day6::part1 (avg 20.089µs, stddev 2.074µs in 24889 runs)
// day6::part2 (avg 203.058542ms, stddev 440.714µs in 3 runs)
// day7::part1 (avg 197.67µs, stddev 17.230µs in 2530 runs)
// day7::part2 (avg 199.364µs, stddev 13.630µs in 2508 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
    puzzle!(day3: part1 = 153469856, part2 = 77055967);
    puzzle!(day4: part1 = 2297, part2 = 1745);
    puzzle!(day5: part1 = 5713, part2 = 5180);
    puzzle!(day6: part1 = 4890, part2 = 1995);
    puzzle!(day7: part1 = 3312271365652, part2 = 509463489296712);
}
