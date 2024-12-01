pub fn part1() {
    let input = include_str!("input/day1.txt");

    let (mut list1, mut list2): (Vec<_>, Vec<_>) = input.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let Some(a) = iter.next() else { panic!("invalid line") };
        let Some(b) = iter.next() else { panic!("invalid line") };
        (a.parse::<isize>().expect("parse int"), b.parse::<isize>().expect("parse int"))
    }).unzip();
    
    list1.sort_unstable();
    list2.sort_unstable();
    let sum: isize = list1.into_iter().zip(list2.into_iter()).map(|(a, b)| (a - b).abs()).sum();
    println!("{sum}");
}