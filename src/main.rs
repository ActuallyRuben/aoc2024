use std::time::Duration;

mod day1;
mod day2;
mod day3;
mod macros;
mod util;

const TIMEOUT: Duration = Duration::from_millis(500);

// Scores:
// day1::part1 (avg 31.515µs, stddev 1.100µs in 15866 runs)
// day1::part2 (avg 42.266µs, stddev 1.181µs in 11830 runs)
// day2::part1 (avg 45.502µs, stddev 1.724µs in 10989 runs)
// day2::part2 (avg 87.779µs, stddev 2.910µs in 5697 runs)
// day3::part1 (avg 31.638µs, stddev 1.331µs in 15804 runs)
// day3::part2 (avg 22.695µs, stddev 0.941µs in 22031 runs)

fn main() {
    puzzle!(day1: part1 = 1579939, part2 = 20351745);
    puzzle!(day2: part1 = 483, part2 = 528);
    puzzle!(day3: part1 = 153469856, part2 = 77055967);
}
