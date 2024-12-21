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
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
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
// day1::part1 (avg 28.81µs, stddev 4.864µs in 17355 runs)
// day1::part2 (avg 50.336µs, stddev 8.176µs in 9934 runs)
// day2::part1 (avg 37.207µs, stddev 4.129µs in 13439 runs)
// day2::part2 (avg 95.226µs, stddev 12.088µs in 5251 runs)
// day3::part1 (avg 19.574µs, stddev 4.499µs in 25544 runs)
// day3::part2 (avg 14.216µs, stddev 3.356µs in 35170 runs)
// day4::part1 (avg 280.885µs, stddev 23.264µs in 1781 runs)
// day4::part2 (avg 65.554µs, stddev 9.263µs in 7628 runs)
// day5::part1 (avg 236.951µs, stddev 19.956µs in 2111 runs)
// day5::part2 (avg 392.152µs, stddev 30.763µs in 1276 runs)
// day6::part1 (avg 21.707µs, stddev 4.208µs in 23034 runs)
// day6::part2 (avg 78.165041ms, stddev 4389.505µs in 7 runs)
// day7::part1 (avg 196.999µs, stddev 30.566µs in 2539 runs)
// day7::part2 (avg 209.771µs, stddev 51.981µs in 2384 runs)
// day8::part1 (avg 28.414µs, stddev 5.171µs in 17597 runs)
// day8::part2 (avg 65.359µs, stddev 7.365µs in 7651 runs)
// day9::part1 (avg 22.015µs, stddev 5.314µs in 22712 runs)
// day9::part2 (avg 610.646864ms, stddev 0.000µs in 1 runs)
// day10::part1 (avg 170.167µs, stddev 17.942µs in 2939 runs)
// day10::part2 (avg 216.383µs, stddev 19.436µs in 2311 runs)
// day11::part1 (avg 185.618µs, stddev 11.207µs in 2694 runs)
// day11::part2 (avg 8.252982ms, stddev 222.788µs in 61 runs)
// day12::part1 (avg 255.89µs, stddev 14.090µs in 1954 runs)
// day12::part2 (avg 440.437µs, stddev 18.791µs in 1136 runs)
// day13::part1 (avg 36.197µs, stddev 3.244µs in 13814 runs)
// day13::part2 (avg 36.312µs, stddev 3.429µs in 13770 runs)
// day14::part1 (avg 24.255µs, stddev 2.805µs in 20614 runs)
// day14::part2 (avg 14.863101ms, stddev 192.317µs in 34 runs)
// day15::part1 (avg 254.378µs, stddev 18.790µs in 1966 runs)
// day15::part2 (avg 392.971µs, stddev 19.774µs in 1273 runs)
// day16::part1 (avg 1.14937ms, stddev 41.199µs in 436 runs)
// day16::part2 (avg 1.694242ms, stddev 51.772µs in 296 runs)
// day18::part1 (avg 81.907µs, stddev 6.974µs in 6105 runs)
// day18::part2 (avg 114.7575ms, stddev 714.217µs in 5 runs)
// day19::part1 (avg 1.574822ms, stddev 38.126µs in 318 runs)
// day19::part2 (avg 12.795965ms, stddev 161.252µs in 40 runs)
// day20::part1 (avg 447.809µs, stddev 21.013µs in 1117 runs)
// day20::part2 (avg 19.479278ms, stddev 268.554µs in 26 runs)
// day21::part1 (avg 9.314µs, stddev 1.638µs in 53678 runs)
// day21::part2 (avg 96.067µs, stddev 7.818µs in 5205 runs)

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
    puzzle!(day18: part1 = 252, part2 = "5,60");
    puzzle!(day19: part1 = 276, part2 = 681226908011510);
    puzzle!(day20: part1 = 1490, part2 = 1011325);
    puzzle!(day21: part1 = 176870, part2 = 223902935165512);
}
