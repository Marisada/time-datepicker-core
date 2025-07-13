pub mod date_constraints;

use derive_builder::Builder;
use derive_getters::Getters;
use time::{OffsetDateTime, PrimitiveDateTime, macros::offset};

use self::date_constraints::HasDateConstraints;

use crate::dialog_view_type::DialogViewType;

/// Configuration for the datepicker.
#[derive(Clone, Default, Debug, Builder, Getters)]
#[builder(setter(strip_option))]
#[builder(default)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct PickerConfig<T: HasDateConstraints + Default + Clone> {
    /// Possible constraints to prevent the user from selecting some dates
    #[getter(skip)]
    date_constraints: T,

    /// Initializes the datepicker to this value
    initial_date: Option<PrimitiveDateTime>,

    /// Initializes the view type to this value
    initial_view_type: DialogViewType,

    /// Selection type, to make it possible to select for example only a year, or only a month.
    selection_type: DialogViewType,
}

impl<T> HasDateConstraints for PickerConfig<T>
where
    T: HasDateConstraints + std::default::Default + Clone,
{
    fn is_datetime_forbidden(&self, date: &PrimitiveDateTime) -> bool {
        self.date_constraints.is_datetime_forbidden(date)
    }

    fn is_day_forbidden(&self, date: &PrimitiveDateTime) -> bool {
        self.date_constraints.is_day_forbidden(date)
    }

    fn is_month_forbidden(&self, year_month_info: &PrimitiveDateTime) -> bool {
        self.date_constraints.is_month_forbidden(year_month_info)
    }

    fn is_year_forbidden(&self, year: &PrimitiveDateTime) -> bool {
        self.date_constraints.is_year_forbidden(year)
    }

    fn is_year_group_forbidden(&self, year: &PrimitiveDateTime) -> bool {
        self.date_constraints.is_year_group_forbidden(year)
    }
}

impl<T> PickerConfigBuilder<T>
where
    T: HasDateConstraints + std::default::Default + Clone,
{
    fn validate(&self) -> Result<(), String> {
        if self.initial_view_type > self.selection_type {
            return Err("initial_view_type can have at most selection_type scale".into());
        }
        if let (Some(Some(initial_date)), Some(date_constraints)) =
            (self.initial_date, &self.date_constraints)
        {
            if date_constraints.is_day_forbidden(&initial_date) {
                return Err(format!(
                    "The initial_date {initial_date} is forbidden by the date_constraints."
                ));
            }
        }
        Ok(())
    }
}

impl<T> PickerConfig<T>
where
    T: HasDateConstraints + std::default::Default + Clone,
{
    pub fn guess_allowed_year_month(&self) -> PrimitiveDateTime {
        if let Some(init_date) = self.initial_date {
            return init_date;
        }
        // if none of the above constraints matched use the current datetime
        let ts_milli = js_sys::Date::now() as i64;
        let local = OffsetDateTime::from_unix_timestamp(ts_milli.saturating_div(1_000))
            .unwrap()
            .to_offset(offset!(+7));
        PrimitiveDateTime::new(local.date(), local.time())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::date_constraints::MockHasDateConstraints;

    use mockall::predicate;
    use time::{Date, Month, Time};

    #[test]
    fn picker_config_initial_view_type_greater_than_selection_type() {
        let config = PickerConfigBuilder::<MockHasDateConstraints>::default()
            .initial_view_type(DialogViewType::Days)
            .selection_type(DialogViewType::Months)
            .build();
        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err().to_string(),
            "initial_view_type can have at most selection_type scale"
        );
    }

    #[test]
    fn picker_config_initial_view_type_equal_to_selection_type() {
        let config = PickerConfigBuilder::<MockHasDateConstraints>::default()
            .initial_view_type(DialogViewType::Months)
            .selection_type(DialogViewType::Months)
            .build();
        assert!(config.is_ok());
    }

    #[test]
    fn picker_config_initial_view_type_smaller_than_selection_type() {
        let config = PickerConfigBuilder::<MockHasDateConstraints>::default()
            .initial_view_type(DialogViewType::Years)
            .selection_type(DialogViewType::Months)
            .build();
        assert!(config.is_ok());
    }

    #[test]
    fn picker_config_initial_date_forbidden() {
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_day_forbidden()
            .returning(|_| true);
        let config = PickerConfigBuilder::<MockHasDateConstraints>::default()
            .initial_date(PrimitiveDateTime::new(
                Date::from_calendar_date(2020, Month::January, 1).expect("invalid date"),
                Time::from_hms(0, 0, 0).expect("invalid time"),
            ))
            .date_constraints(date_constraints_mock)
            .build();
        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err().to_string(),
            "The initial_date 2020-01-01 0:00:00.0 is forbidden by the date_constraints."
        );
    }

    /// Test utility function to inject the mocked date constraints directly into the `PickerConfig`.
    fn create_picker_config_with_mocked_date_constraints<T>(
        builder: PickerConfigBuilder<T>,
        mock_constraints: T,
    ) -> PickerConfig<T>
    where
        T: HasDateConstraints + Clone + std::default::Default,
    {
        let config = builder.build().unwrap();
        PickerConfig {
            date_constraints: mock_constraints,
            initial_date: *config.initial_date(),
            initial_view_type: *config.initial_view_type(),
            selection_type: *config.selection_type(),
        }
    }

    #[test]
    fn test_is_day_forbidden() {
        let date = PrimitiveDateTime::new(
            Date::from_calendar_date(2020, Month::January, 1).expect("invalid date"),
            Time::from_hms(0, 0, 0).expect("invalid time"),
        );
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_day_forbidden()
            .with(predicate::eq(date))
            .times(1)
            .returning(|_| true);
        let builder = PickerConfigBuilder::default();
        let config =
            create_picker_config_with_mocked_date_constraints(builder, date_constraints_mock);
        assert!(config.is_day_forbidden(&date));
    }

    #[test]
    fn test_is_month_forbidden() {
        let year_month = PrimitiveDateTime::new(
            Date::from_calendar_date(2000, Month::February, 24).expect("invalid date"),
            Time::from_hms(0, 0, 0).expect("invalid time"),
        );
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_month_forbidden()
            .with(predicate::eq(year_month.clone()))
            .times(1)
            .returning(|_| true);
        let builder = PickerConfigBuilder::default();
        let config =
            create_picker_config_with_mocked_date_constraints(builder, date_constraints_mock);
        assert!(config.is_month_forbidden(&year_month));
    }

    #[test]
    fn test_is_year_forbidden() {
        let year = PrimitiveDateTime::new(
            Date::from_calendar_date(2000, Month::January, 1).expect("invalid date"),
            Time::from_hms(0, 0, 0).expect("invalid time"),
        );
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_year_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(|_| true);
        let builder = PickerConfigBuilder::default();
        let config =
            create_picker_config_with_mocked_date_constraints(builder, date_constraints_mock);
        assert!(config.is_year_forbidden(&year));
    }

    #[test]
    fn test_is_year_group_forbidden() {
        let year = PrimitiveDateTime::new(
            Date::from_calendar_date(2000, Month::January, 1).expect("invalid date"),
            Time::from_hms(0, 0, 0).expect("invalid time"),
        );
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_year_group_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(|_| true);
        let builder = PickerConfigBuilder::default();
        let config =
            create_picker_config_with_mocked_date_constraints(builder, date_constraints_mock);
        assert!(config.is_year_group_forbidden(&year));
    }

    #[test]
    fn guess_allowed_year_month_with_initial_date() {
        let initial_date = PrimitiveDateTime::new(
            Date::from_calendar_date(2020, Month::March, 24).expect("invalid date"),
            Time::from_hms(0, 0, 0).expect("invalid time"),
        );
        let config = PickerConfigBuilder::<MockHasDateConstraints>::default()
            .initial_date(initial_date)
            .build()
            .unwrap();
        let expected = initial_date;
        assert_eq!(expected, config.guess_allowed_year_month());
    }
}
