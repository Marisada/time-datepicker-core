use time::{Date, Month, PrimitiveDateTime, Time};

use crate::{
    config::date_constraints::HasDateConstraints, dialog_view_type::DialogViewType,
    viewed_date::ViewedDate,
};

/// Returns true if the "previous" button should be displayed.
pub fn should_display_previous_button<T: HasDateConstraints>(
    dialog_view_type: &DialogViewType,
    viewed_date: &PrimitiveDateTime,
    config: &T,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&viewed_date.previous_month()),
        DialogViewType::Months => !config.is_year_forbidden(&viewed_date.previous_year()),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(&viewed_date.previous_year_group())
        }
    }
}

/// Returns true if the "next" button should be displayed.
pub fn should_display_next_button<T: HasDateConstraints>(
    dialog_view_type: &DialogViewType,
    viewed_date: &PrimitiveDateTime,
    config: &T,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&viewed_date.next_month()),
        DialogViewType::Months => !config.is_year_forbidden(&viewed_date.next_year()),
        DialogViewType::Years => !config.is_year_group_forbidden(&viewed_date.next_year_group()),
    }
}

/// Returns a `Date`
pub fn from_ymd(year: i32, month: u8, day: u8) -> Date {
    let m = Month::try_from(month).expect("invalid or out-of-range month");
    Date::from_calendar_date(year, m, day).expect("invalid or out-of-range date")
}

/// Returns a `PrimitiveDateTime`
pub fn from_ymdhm(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> PrimitiveDateTime {
    let m = Month::try_from(month).expect("invalid or out-of-range month");
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, m, day).expect("invalid or out-of-range date"),
        Time::from_hms(hour, minute, 0).expect("invalid or out-of-range time"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::date_constraints::MockHasDateConstraints;

    use crate::rstest_utils::create_datetime;
    use mockall::predicate;
    use rstest::*;

    #[fixture(viewed_date=create_datetime(1990, 1, 1, 0, 0), retval=false)]
    fn month_forbidden(viewed_date: PrimitiveDateTime, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::default();
        mock.expect_is_month_forbidden()
            .with(predicate::eq(viewed_date))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = create_datetime(1990, 1, 1, 0, 0), retval = false)]
    fn year_forbidden(year: PrimitiveDateTime, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::default();
        mock.expect_is_year_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = create_datetime(1990, 1, 1, 0, 0), retval = false)]
    fn year_group_forbidden(year: PrimitiveDateTime, retval: bool) -> MockHasDateConstraints {
        let mut mock = MockHasDateConstraints::default();
        mock.expect_is_year_group_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[rstest(
        expected, dialog_view_type, viewed_date, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_datetime(1990, 2, 16, 0, 0), month_forbidden(create_datetime(1990, 1, 1, 0, 0), true)),
        case::month_allowed(true, DialogViewType::Days, create_datetime(1990, 3, 25, 0, 0), month_forbidden(create_datetime(1990, 2, 1, 0, 0), false)),
        case::year_forbidden(false, DialogViewType::Months, create_datetime(1990, 4, 26, 0, 0), year_forbidden(create_datetime(1989, 1, 1, 0, 0), true)),
        case::year_allowed(true, DialogViewType::Months, create_datetime(1990, 7, 18, 0, 0), year_forbidden(create_datetime(1989, 1, 1, 0, 0), false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_datetime(1990, 2, 16, 0, 0), year_group_forbidden(create_datetime(1979, 1, 1, 0, 0), true)),
        case::year_group_allowed(true, DialogViewType::Years, create_datetime(1990, 2, 18, 0, 0), year_group_forbidden(create_datetime(1979, 1, 1, 0, 0), false)),
    )]
    fn test_should_display_previous_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        viewed_date: PrimitiveDateTime,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_previous_button(&dialog_view_type, &viewed_date, &mock_constraints)
        );
    }

    #[rstest(
        expected, dialog_view_type, viewed_date, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_datetime(1990, 2, 18, 0, 0), month_forbidden(create_datetime(1990, 3, 1, 0, 0), true)),
        case::month_allowed(true, DialogViewType::Days, create_datetime(1990, 2, 15, 0, 0), month_forbidden(create_datetime(1990, 3, 1, 0, 0), false)),
        case::year_forbidden(false, DialogViewType::Months, create_datetime(1990, 8, 16, 0, 0), year_forbidden(create_datetime(1991, 1, 1, 0, 0), true)),
        case::year_allowed(true, DialogViewType::Months, create_datetime(1990, 4, 21, 0, 0), year_forbidden(create_datetime(1991, 1, 1, 0, 0), false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_datetime(1990, 11, 26, 0, 0), year_group_forbidden(create_datetime(2000, 1, 1, 0, 0), true)),
        case::year_group_allowed(true, DialogViewType::Years, create_datetime(1990, 12, 23, 0, 0), year_group_forbidden(create_datetime(2000, 1, 1, 0, 0), false)),
    )]
    fn test_should_display_next_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        viewed_date: PrimitiveDateTime,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_next_button(&dialog_view_type, &viewed_date, &mock_constraints)
        );
    }
}
