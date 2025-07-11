/// Types of views for the datepicker.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DialogViewType {
    /// YEARS_IN_YEAR_SELECTION Years, from a year which modulo `% 20 == 0`
    Years = 1,
    /// 1 full year with the selection of a month
    Months = 2,
    /// 1 full month with the selection of a day
    #[default]
    Days = 3,
}

impl DialogViewType {
    /// Returns the larger view type, if such exists, otherwise returns None
    pub const fn larger_type(&self) -> Option<Self> {
        match self {
            DialogViewType::Years => None,
            DialogViewType::Months => Some(DialogViewType::Years),
            DialogViewType::Days => Some(DialogViewType::Months),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest(
        expected, input, //
        case::years(None, DialogViewType::Years),
        case::months(Some(DialogViewType::Years), DialogViewType::Months),
        case::days(Some(DialogViewType::Months), DialogViewType::Days),
    )]
    fn larger_type(expected: Option<DialogViewType>, input: DialogViewType) {
        assert_eq!(expected, input.larger_type());
    }
}
