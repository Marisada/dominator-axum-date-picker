pub mod date_constraints;
use self::date_constraints::HasDateConstraints;

use crate::dialog_view_type::DialogViewType;

use time::{Date, Time};

use picker_util::js_now;

/// Configuration for the datepicker.
#[derive(Default, Debug, Builder, Getters)]
#[builder(setter(strip_option))]
#[builder(default)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct PickerConfig<T: HasDateConstraints + Default + Clone> {
    /// possible constraints to prevent the user from selecting some dates
    #[getter(skip)]
    date_constraints: T,

    /// initializes the datepicker to this value
    initial_date: Option<Date>,
    initial_time: Option<Time>,

    /// initializes the view type to this value
    initial_view_type: DialogViewType,

    /// selection type, to make it possible to select for example only a year, or only a month.
    selection_type: DialogViewType,
}

impl<T: HasDateConstraints + std::default::Default + Clone> HasDateConstraints for PickerConfig<T> {
    fn is_day_forbidden(&self, date: &Date) -> bool {
        self.date_constraints.is_day_forbidden(date)
    }

    fn is_month_forbidden(&self, year_month_info: &Date) -> bool {
        self.date_constraints.is_month_forbidden(year_month_info)
    }

    fn is_year_forbidden(&self, year: i32) -> bool {
        self.date_constraints.is_year_forbidden(year)
    }

    fn is_year_group_forbidden(&self, year: i32) -> bool {
        self.date_constraints.is_year_group_forbidden(year)
    }
}

impl<T: HasDateConstraints + std::default::Default + Clone> PickerConfigBuilder<T> {
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

impl<T: HasDateConstraints + std::default::Default + Clone> PickerConfig<T> {
    pub fn guess_allowed_year_month(&self) -> Date {
        if let Some(init_date) = self.initial_date {
            return init_date;
        }
        // if none of the above constraints matched use the current_date
        js_now().date()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::date_constraints::MockHasDateConstraints;

    use mockall::predicate;
    use time::Month;

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
            .initial_date(Date::from_calendar_date(2020, Month::January, 1).expect("invalid date"))
            .date_constraints(date_constraints_mock)
            .build();
        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err().to_string(),
            "The initial_date 2020-01-01 is forbidden by the date_constraints."
        );
    }

    /// Test utility function to inject the mocked date constraints directly into the `PickerConfig`.
    fn create_picker_config_with_mocked_date_constraints<
        T: HasDateConstraints + Clone + std::default::Default,
    >(
        builder: PickerConfigBuilder<T>,
        mock_constraints: T,
    ) -> PickerConfig<T> {
        let config = builder.build().unwrap();
        PickerConfig {
            date_constraints: mock_constraints,
            initial_date: *config.initial_date(),
            initial_time: *config.initial_time(),
            initial_view_type: *config.initial_view_type(),
            selection_type: *config.selection_type(),
        }
    }

    #[test]
    fn test_is_day_forbidden() {
        let date = Date::from_calendar_date(2020, Month::January, 1).expect("invalid date");
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
        let year_month = Date::from_calendar_date(2000, Month::February, 24).expect("invalid date");
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
        let year = 2000i32;
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_year_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(|_| true);
        let builder = PickerConfigBuilder::default();
        let config =
            create_picker_config_with_mocked_date_constraints(builder, date_constraints_mock);
        assert!(config.is_year_forbidden(year));
    }

    #[test]
    fn test_is_year_group_forbidden() {
        let year = 2000i32;
        let mut date_constraints_mock = MockHasDateConstraints::new();
        date_constraints_mock
            .expect_is_year_group_forbidden()
            .with(predicate::eq(year))
            .times(1)
            .returning(|_| true);
        let builder = PickerConfigBuilder::default();
        let config =
            create_picker_config_with_mocked_date_constraints(builder, date_constraints_mock);
        assert!(config.is_year_group_forbidden(year));
    }

    #[test]
    fn guess_allowed_year_month_with_initial_date() {
        let initial_date = Date::from_calendar_date(2020, Month::March, 24).expect("invalid date");
        let config = PickerConfigBuilder::<MockHasDateConstraints>::default()
            .initial_date(initial_date)
            .build()
            .unwrap();
        let expected = initial_date;
        assert_eq!(expected, config.guess_allowed_year_month());
    }
}
