#[macro_export]
macro_rules! solution {
    ($name:expr, $solve:expr) => {
        use humantime::format_duration;
        use owo_colors::OwoColorize;
        use std::time::{Duration, Instant};

        fn main() {
            let (solution, time) = get_time(|| $solve);

            println!(
                "{name}: {solution} in {time}",
                name = $name,
                solution = solution.fg_rgb::<255, 63, 128>(),
                time = format_duration(time).fg_rgb::<101, 252, 218>(),
            );
        }

        fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
            let start = Instant::now();
            let result = f();
            let time = start.elapsed();

            (result, time)
        }
    };
}

#[macro_export]
macro_rules! test {
    ($({$test_name:ident, $fun:expr, $answer:expr}),+) => {
        #[cfg(test)]
        mod test {
            use super::*;

            $(
                #[test]
                #[allow(clippy::bool_assert_comparison)]
                fn $test_name() {
                    assert_eq!($fun, $answer);
                }
            )+
        }
    };
}
