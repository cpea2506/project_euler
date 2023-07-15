use std::time::{Duration, Instant};

#[doc(hidden)]
pub fn __get_time<T>(func: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = func();
    let time = start.elapsed();

    (result, time)
}

#[macro_export]
macro_rules! run {
    ($name:expr, $solution:expr) => {
        use humantime::format_duration;
        use owo_colors::OwoColorize;

        fn main() {
            let (solution, time) = $crate::__get_time(|| $solution);

            println!(
                "{name}: {solution:?} in {time}",
                name = $name,
                solution = solution.fg_rgb::<255, 63, 128>(),
                time = format_duration(time).fg_rgb::<101, 252, 218>(),
            );
        }
    };
}

#[macro_export]
macro_rules! test {
    ($mod_name:ident{$({$test_name:ident, $fun:expr, $answer:expr}),+ $(,)?}) => {
        #[cfg(test)]
        mod $mod_name {
            use super::*;

            $(
                #[test]
                fn $test_name() {
                    assert_eq!($fun, $answer);
                }
            )+
        }
    };

    ($mod_name:ident{$($test_name:ident $fun:expr),+ $(,)?}) => {
        #[cfg(test)]
        mod $mod_name {
            use super::*;

            $(
                #[test]
                fn $test_name() {
                    $fun
                }
            )+
        }
    };
}
