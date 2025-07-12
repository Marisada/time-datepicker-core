use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use time::{Date, Month, PrimitiveDateTime, Time, Weekday};
use time_datepicker_core::config::{
    PickerConfig, PickerConfigBuilder,
    date_constraints::{DateConstraints, DateConstraintsBuilder, HasDateConstraints},
};

criterion_group!(
    benches,
    is_year_forbidden_in_disabled_year,
    is_day_forbidden_day_allowed,
    is_day_forbidden_sooner_than_min_date,
    is_day_forbidden_later_than_max_date,
    is_day_forbidden_on_disabled_weekday,
    is_day_forbidden_in_disabled_month,
    is_day_forbidden_on_disabled_monthly_date,
    is_day_forbidden_on_disabled_yearly_date,
    is_day_forbidden_on_disabled_unique_date,
);
criterion_main!(benches);

fn create_config() -> PickerConfig<DateConstraints> {
    PickerConfigBuilder::default()
        .initial_date(PrimitiveDateTime::new(
            Date::from_calendar_date(2020, Month::December, 15).unwrap(),
            Time::from_hms(23, 55, 0).unwrap(),
        ))
        .date_constraints(
            DateConstraintsBuilder::default()
                .min_datetime(PrimitiveDateTime::new(
                    Date::from_calendar_date(2020, Month::December, 1).unwrap(),
                    Time::from_hms(0, 0, 0).unwrap(),
                ))
                .max_datetime(PrimitiveDateTime::new(
                    Date::from_calendar_date(2022, Month::December, 14).unwrap(),
                    Time::from_hms(0, 0, 0).unwrap(),
                ))
                .disabled_weekdays(
                    [Weekday::Saturday, Weekday::Sunday]
                        .iter()
                        .cloned()
                        .collect(),
                )
                .disabled_months([Month::July, Month::August].iter().cloned().collect())
                .disabled_years([2021].iter().cloned().collect())
                .disabled_monthly_dates([13].iter().cloned().collect())
                .disabled_yearly_dates(vec![
                    Date::from_calendar_date(1, Month::December, 24).unwrap(),
                    Date::from_calendar_date(1, Month::December, 25).unwrap(),
                    Date::from_calendar_date(1, Month::December, 26).unwrap(),
                ])
                .disabled_unique_dates(
                    [Date::from_calendar_date(2020, Month::December, 8).unwrap()]
                        .iter()
                        .cloned()
                        .collect(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

#[allow(dead_code)]
fn is_day_forbidden_day_allowed(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2020, Month::December, 9).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_day_allowed", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_sooner_than_min_date(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2020, Month::November, 30).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_sooner_than_min_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_later_than_max_date(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2023, Month::February, 15).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_later_than_max_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_weekday(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2020, Month::December, 12).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_weekday", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_in_disabled_month(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2022, Month::July, 12).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_in_disabled_month", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_in_disabled_year(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2021, Month::December, 9).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_in_disabled_year", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_monthly_date(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2022, Month::January, 13).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_monthly_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_yearly_date(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2020, Month::December, 24).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_yearly_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_unique_date(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2020, Month::December, 8).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_unique_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_year_forbidden_in_disabled_year(c: &mut Criterion) {
    let start_date = PrimitiveDateTime::new(
        Date::from_calendar_date(2021, Month::December, 31).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let config = create_config();
    c.bench_function("is_year_forbidden_in_disabled_year", |b| {
        b.iter(|| config.is_year_forbidden(black_box(&start_date)))
    });
}
