use time::{Date, Month};

use crate::{
    config::date_constraints::HasDateConstraints,
    dialog_view_type::DialogViewType,
    viewed_date::ViewedDate,
};

/// Returns true if the "previous" button should be displayed.
pub fn should_display_previous_button<T: HasDateConstraints>(
    dialog_view_type: &DialogViewType,
    viewed_date: &Date,
    config: &T,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&viewed_date.previous_month()),
        DialogViewType::Months => !config.is_year_forbidden(viewed_date.previous_year().year()),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(viewed_date.previous_year_group().year())
        }
    }
}

/// Returns true if the "next" button should be displayed.
pub fn should_display_next_button<T: HasDateConstraints>(
    dialog_view_type: &DialogViewType,
    viewed_date: &Date,
    config: &T,
) -> bool {
    match dialog_view_type {
        DialogViewType::Days => !config.is_month_forbidden(&viewed_date.next_month()),
        DialogViewType::Months => !config.is_year_forbidden(viewed_date.next_year().year()),
        DialogViewType::Years => {
            !config.is_year_group_forbidden(viewed_date.next_year_group().year())
        }
    }
}

/// Returns a `Date` (simple utility function because the one in `Date` got deprecated).
pub fn from_ymd(year: i32, month: u8, day: u8) -> Date {
    let m = Month::try_from(month).expect("invalid or out-of-range month");
    Date::from_calendar_date(year, m, day).expect("invalid or out-of-range date")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::date_constraints::MockHasDateConstraints, viewed_date::YearNumber};

    use crate::rstest_utils::create_date;
    use mockall::predicate;
    use rstest::*;

    #[fixture(viewed_date=create_date(1990, 1, 1), retval=false)]
    fn month_forbidden(viewed_date: Date, retval: bool) -> MockHasDateConstraints  {
        let mut mock = MockHasDateConstraints::default();
        mock.expect_is_month_forbidden()
            .with(predicate::eq(viewed_date))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = 1990, retval = false)]
    fn year_forbidden(year: YearNumber, retval: bool) -> MockHasDateConstraints  {
        let mut mock = MockHasDateConstraints::default();
        mock.expect_is_year_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[fixture(year = 1990, retval = false)]
    fn year_group_forbidden(year: YearNumber, retval: bool) -> MockHasDateConstraints  {
        let mut mock = MockHasDateConstraints::default();
        mock.expect_is_year_group_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(move |_| retval);
        mock
    }

    #[rstest(
        expected, dialog_view_type, viewed_date, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_date(1990, 2, 16), month_forbidden(create_date(1990, 1, 1), true)),
        case::month_allowed(true, DialogViewType::Days, create_date(1990, 3, 25), month_forbidden(create_date(1990, 2, 1), false)),
        case::year_forbidden(false, DialogViewType::Months, create_date(1990, 4, 26), year_forbidden(1989, true)),
        case::year_allowed(true, DialogViewType::Months, create_date(1990, 7, 18), year_forbidden(1989, false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_date(1990, 2, 16), year_group_forbidden(1979, true)),
        case::year_group_allowed(true, DialogViewType::Years, create_date(1990, 2, 18), year_group_forbidden(1979, false)),
    )]
    fn test_should_display_previous_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        viewed_date: Date,
        mock_constraints: MockHasDateConstraints,
    ) {
        assert_eq!(
            expected,
            should_display_previous_button(&dialog_view_type, &viewed_date, &mock_constraints)
        );
    }

    #[rstest(
        expected, dialog_view_type, viewed_date, mock_constraints, //
        case::month_forbidden(false, DialogViewType::Days, create_date(1990, 2, 18), month_forbidden(create_date(1990, 3, 1), true)),
        case::month_allowed(true, DialogViewType::Days, create_date(1990, 2, 15), month_forbidden(create_date(1990, 3, 1), false)),
        case::year_forbidden(false, DialogViewType::Months, create_date(1990, 8, 16), year_forbidden(1991, true)),
        case::year_allowed(true, DialogViewType::Months, create_date(1990, 4, 21), year_forbidden(1991, false)),
        case::year_group_forbidden(false, DialogViewType::Years, create_date(1990, 11, 26), year_group_forbidden(2000, true)),
        case::year_group_allowed(true, DialogViewType::Years, create_date(1990, 12, 23), year_group_forbidden(2000, false)),
    )]
    fn test_should_display_next_button(
        expected: bool,
        dialog_view_type: DialogViewType,
        viewed_date: Date,
        mock_constraints: MockHasDateConstraints ,
    ) {
        assert_eq!(
            expected,
            should_display_next_button(&dialog_view_type, &viewed_date, &mock_constraints)
        );
    }
}
