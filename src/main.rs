use std::time::Duration;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
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
// day1::part1 (avg 27.932µs, stddev 3.272µs in 17901 runs)
// day1::part2 (avg 48.217µs, stddev 4.146µs in 10370 runs)
// day2::part1 (avg 35.604µs, stddev 1.910µs in 14044 runs)
// day2::part2 (avg 87.457µs, stddev 7.004µs in 5718 runs)
// day3::part1 (avg 18.21µs, stddev 3.057µs in 27457 runs)
// day3::part2 (avg 13.493µs, stddev 2.289µs in 37054 runs)
// day4::part1 (avg 268.03µs, stddev 13.770µs in 1866 runs)
// day4::part2 (avg 62.373µs, stddev 5.333µs in 8017 runs)
// day5::part1 (avg 231.165µs, stddev 12.370µs in 2163 runs)
// day5::part2 (avg 378.871µs, stddev 20.990µs in 1320 runs)
// day6::part1 (avg 20.012µs, stddev 3.722µs in 24985 runs)
// day6::part2 (avg 72.698298ms, stddev 645.855µs in 7 runs)
// day7::part1 (avg 200.368µs, stddev 33.159µs in 2496 runs)
// day7::part2 (avg 201.318µs, stddev 25.852µs in 2484 runs)
// day8::part1 (avg 26.789µs, stddev 3.938µs in 18664 runs)
// day8::part2 (avg 65.917µs, stddev 9.231µs in 7586 runs)
// day9::part1 (avg 21.602µs, stddev 4.737µs in 23146 runs)
// day9::part2 (avg 602.028244ms, stddev 0.000µs in 1 runs)
// day10::part1 (avg 164.52µs, stddev 14.087µs in 3040 runs)
// day10::part2 (avg 211.037µs, stddev 17.265µs in 2370 runs)
// day11::part1 (avg 188.004µs, stddev 15.804µs in 2660 runs)
// day11::part2 (avg 8.506414ms, stddev 186.386µs in 59 runs)
// day12::part1 (avg 263.765µs, stddev 14.648µs in 1896 runs)
// day12::part2 (avg 466.671µs, stddev 20.039µs in 1072 runs)
// day13::part1 (avg 36.456µs, stddev 4.256µs in 13715 runs)
// day13::part2 (avg 37.14µs, stddev 5.019µs in 13463 runs)
// day14::part1 (avg 26.078µs, stddev 2.363µs in 19173 runs)
// day14::part2 (avg 14.375035ms, stddev 66.855µs in 35 runs)
// day15::part1 (avg 255.925µs, stddev 15.077µs in 1954 runs)
// day15::part2 (avg 379.209µs, stddev 17.488µs in 1319 runs)
// day16::part1 (avg 1.068117ms, stddev 32.279µs in 469 runs)
// day16::part2 (avg 1.630709ms, stddev 39.135µs in 307 runs)

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
    puzzle!(day10: part1 = 489, part2 = 1086);
    puzzle!(day11: part1 = 193899, part2 = 229682160383225);
    puzzle!(day12: part1 = 1381056, part2 = 834828);
    puzzle!(day13: part1 = 29522, part2 = 101214869433312);
    puzzle!(day14: part1 = 228410028, part2 = 8258);
    puzzle!(day15: part1 = 1552463, part2 = 1554058);
    puzzle!(day16: part1 = 98520, part2 = 609);
    // puzzle!(day17: part1, part2);
}
