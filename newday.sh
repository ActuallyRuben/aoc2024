#!/bin/sh

set -eu


if [ $# -ne 1  ]
then
  echo "usage $0 <day_number>"
  exit 1
fi

if [ -e "src/day$1.rs" ]
then
  echo "day already exists"
fi

echo "day $1"
echo "pub fn part1(input: &str) -> usize {
    0
}" >"src/day$1.rs"
sed -i "s/}/    puzzle!(day$1: part1);\n}/" src/main.rs
sed -i "s/mod macros/mod day$1;\nmod macros/" src/main.rs
sed -i "s/pub(crate) mod util/pub mod day$1;\npub(crate) mod util/" src/lib.rs
cargo fmt
touch "src/input/day$1.txt"
git add "src/day$1.rs" "src/input/day$1.txt"
