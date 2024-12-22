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
mod day22;
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
// day1::part1 (avg 28.111µs, stddev 4.877µs in 17787 runs)
// day1::part2 (avg 48.308µs, stddev 5.353µs in 10351 runs)
// day2::part1 (avg 36.999µs, stddev 3.226µs in 13514 runs)
// day2::part2 (avg 90.505µs, stddev 6.403µs in 5525 runs)
// day3::part1 (avg 16.832µs, stddev 2.482µs in 29705 runs)
// day3::part2 (avg 13.642µs, stddev 2.663µs in 36651 runs)
// day4::part1 (avg 264.233µs, stddev 13.113µs in 1893 runs)
// day4::part2 (avg 63.774µs, stddev 6.640µs in 7841 runs)
// day5::part1 (avg 236.307µs, stddev 15.471µs in 2116 runs)
// day5::part2 (avg 382.826µs, stddev 20.281µs in 1307 runs)
// day6::part1 (avg 20.323µs, stddev 2.487µs in 24603 runs)
// day6::part2 (avg 72.736436ms, stddev 715.653µs in 7 runs)
// day7::part1 (avg 210.239µs, stddev 29.705µs in 2379 runs)
// day7::part2 (avg 208.859µs, stddev 23.293µs in 2394 runs)
// day8::part1 (avg 26.175µs, stddev 2.811µs in 19102 runs)
// day8::part2 (avg 63.102µs, stddev 4.815µs in 7924 runs)
// day9::part1 (avg 21.123µs, stddev 2.814µs in 23670 runs)
// day9::part2 (avg 609.888903ms, stddev 0.000µs in 1 runs)
// day10::part1 (avg 166.241µs, stddev 12.217µs in 3008 runs)
// day10::part2 (avg 206.58µs, stddev 12.476µs in 2421 runs)
// day11::part1 (avg 186.52µs, stddev 13.669µs in 2681 runs)
// day11::part2 (avg 8.26771ms, stddev 103.496µs in 61 runs)
// day12::part1 (avg 262.639µs, stddev 17.943µs in 1904 runs)
// day12::part2 (avg 460.015µs, stddev 30.174µs in 1087 runs)
// day13::part1 (avg 36.729µs, stddev 3.346µs in 13613 runs)
// day13::part2 (avg 36.743µs, stddev 3.166µs in 13608 runs)
// day14::part1 (avg 24.81µs, stddev 3.441µs in 20153 runs)
// day14::part2 (avg 14.770999ms, stddev 275.628µs in 34 runs)
// day15::part1 (avg 260.114µs, stddev 13.334µs in 1923 runs)
// day15::part2 (avg 379.814µs, stddev 17.545µs in 1317 runs)
// day16::part1 (avg 1.198066ms, stddev 30.139µs in 418 runs)
// day16::part2 (avg 1.688799ms, stddev 38.170µs in 297 runs)
// day17::part1 (avg 618ns, stddev 0.308µs in 807983 runs)
// day17::part2 (avg 220.734896ms, stddev 516.549µs in 3 runs)
// day18::part1 (avg 77.599µs, stddev 6.008µs in 6444 runs)
// day18::part2 (avg 113.334134ms, stddev 397.329µs in 5 runs)
// day19::part1 (avg 1.585464ms, stddev 36.638µs in 316 runs)
// day19::part2 (avg 12.758442ms, stddev 96.971µs in 40 runs)
// day20::part1 (avg 463.772µs, stddev 18.071µs in 1079 runs)
// day20::part2 (avg 19.496802ms, stddev 175.588µs in 26 runs)
// day21::part1 (avg 9.401µs, stddev 1.382µs in 53182 runs)
// day21::part2 (avg 96.003µs, stddev 5.651µs in 5209 runs)
// day22::part1 (avg 8.176333ms, stddev 64.129µs in 62 runs)
// day22::part2 (avg 286.374824ms, stddev 276.948µs in 2 runs)

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
    puzzle!(day17: part1 = "2,4,1,2,7,5,4", part2 = 37221274271220);
    puzzle!(day18: part1 = 252, part2 = "5,60");
    puzzle!(day19: part1 = 276, part2 = 681226908011510);
    puzzle!(day20: part1 = 1490, part2 = 1011325);
    puzzle!(day21: part1 = 176870, part2 = 223902935165512);
    puzzle!(day22: part1 = 19241711734, part2 = 2058);
}
