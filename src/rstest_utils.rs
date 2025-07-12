use rstest::fixture;
use time::{Date, PrimitiveDateTime};

use crate::{
    utils::{from_ymd, from_ymdhm},
    viewed_date::{DayNumber, MonthNumber, YearNumber},
};

#[fixture(year = 1990, month = 1, day = 1)]
pub fn create_date(year: YearNumber, month: MonthNumber, day: DayNumber) -> Date {
    from_ymd(year, month, day)
}

#[fixture(year = 1990, month = 1, day = 1, hour = 1, minute = 1)]
pub fn create_datetime(
    year: YearNumber,
    month: MonthNumber,
    day: DayNumber,
    hour: u8,
    minute: u8,
) -> PrimitiveDateTime {
    from_ymdhm(year, month, day, hour, minute)
}
