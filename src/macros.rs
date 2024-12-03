#[macro_export]
macro_rules! puzzle {
    ($day:ident: $($part:ident $(= $sol:expr)?),+$(,)?) => {
        $(
            puzzle_part!($day: $part $(= $sol)?);
        )+
    };
}

#[macro_export]
macro_rules! puzzle_part {
    ($day:ident: $part:ident = $solution:expr) => {{
        if cfg!(feature = "bench") {
            let do_run = || {
                let mut total_time = std::time::Duration::from_micros(0);
                let solution = $solution;
                let mut times: Vec<std::time::Duration> = Vec::new();
                while total_time < $crate::TIMEOUT {
                    let start = std::time::Instant::now();
                    let input = std::hint::black_box(include_str!(concat!(
                        "input/",
                        stringify!($day),
                        ".txt"
                    )));
                    let sol2 = std::hint::black_box($crate::$day::$part(input));
                    let end = std::time::Instant::now();
                    if solution != sol2 {
                        println!(
                            "{}::{} failed on run {} ({:?} != {:?})",
                            stringify!($day),
                            stringify!($part),
                            times.len(),
                            sol2,
                            solution,
                        );
                        return;
                    }
                    times.push(end - start);
                    total_time += end - start;
                }
                let mean_time = total_time / times.len() as u32;
                let mean_nanos = mean_time.as_nanos();
                let std_dev = (times
                    .iter()
                    .map(|time| {
                        let nanos = time.as_nanos();
                        (nanos as f64 - mean_nanos as f64).powi(2)
                    })
                    .sum::<f64>()
                    / times.len() as f64
                    / 1000000.0)
                    .sqrt();
                println!(
                    "{}::{} (avg {:?}, stddev {:.3}µs in {} runs)",
                    stringify!($day),
                    stringify!($part),
                    mean_time,
                    std_dev,
                    times.len()
                );
            };
            do_run();
        }
    };};
    ($day:ident: $part:ident) => {{
        let start = std::time::Instant::now();
        let input = std::hint::black_box(include_str!(concat!("input/", stringify!($day), ".txt")));
        let solution = $crate::$day::$part(input);
        let end = std::time::Instant::now();
        println!(
            "{}::{} = {:?} (took {:?})",
            stringify!($day),
            stringify!($part),
            solution,
            end - start
        );
    };};
}
