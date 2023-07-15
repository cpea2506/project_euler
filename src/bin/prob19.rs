// Counting Sundays

use chrono::{prelude::*, Months, ParseResult};

const DATE_FORMAT: &str = "%d/%m/%Y";

/// The current implementation is true only if the day starts from 1
fn sunday_in_1st_count(start: &str, end: &str) -> ParseResult<usize> {
    let mut start = NaiveDate::parse_from_str(start, DATE_FORMAT)?;
    let end = NaiveDate::parse_from_str(end, DATE_FORMAT)?;

    let mut count = 0;

    while let Some(next_day) = start.checked_add_months(Months::new(1)) {
        if start >= end {
            break;
        }

        if matches!(start.weekday(), Weekday::Sun) {
            count += 1;
        }

        start = next_day;
    }

    Ok(count)
}

pj_euler::run!(
    "Counting Sundays",
    sunday_in_1st_count("1/1/1901", "31/12/2000")
);

pj_euler::test!(
    counting_sunday {
        {from_1_1_1900_to_1_1_1910, sunday_in_1st_count("1/1/1900", "1/1/1910"), Ok(18)},
        {from_1_1_2000_to_1_1_2020, sunday_in_1st_count("1/1/2000", "1/1/2020"), Ok(35)},
    }
);
