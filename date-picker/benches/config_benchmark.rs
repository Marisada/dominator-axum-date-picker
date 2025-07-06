use time::{Month, Date, Weekday};
use date_picker::config::{
    date_constraints::{DateConstraints, DateConstraintsBuilder, HasDateConstraints},
    PickerConfig, PickerConfigBuilder,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
        .initial_date(Date::from_calendar_date(2020, Month::December, 15).unwrap())
        .date_constraints(
            DateConstraintsBuilder::default()
                .min_date(Date::from_calendar_date(2020, Month::December, 1).unwrap())
                .max_date(Date::from_calendar_date(2022, Month::December, 14).unwrap())
                .disabled_weekdays([Weekday::Saturday, Weekday::Sunday].iter().cloned().collect())
                .disabled_months([Month::July, Month::August].iter().cloned().collect())
                .disabled_years([2021].iter().cloned().collect())
                .disabled_monthly_dates([13].iter().cloned().collect())
                .disabled_yearly_dates(vec![
                    Date::from_calendar_date(1, Month::December, 24).unwrap(),
                    Date::from_calendar_date(1, Month::December, 25).unwrap(),
                    Date::from_calendar_date(1, Month::December, 26).unwrap(),
                ])
                .disabled_unique_dates([Date::from_calendar_date(2020, Month::December, 8).unwrap()].iter().cloned().collect())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

#[allow(dead_code)]
fn is_day_forbidden_day_allowed(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2020, Month::December, 9).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_day_allowed", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_sooner_than_min_date(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2020, Month::November, 30).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_sooner_than_min_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_later_than_max_date(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2023, Month::February, 15).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_later_than_max_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_weekday(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2020, Month::December, 12).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_weekday", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_in_disabled_month(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2022, Month::July, 12).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_in_disabled_month", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_in_disabled_year(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2021, Month::December, 9).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_in_disabled_year", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_monthly_date(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2022, Month::January, 13).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_monthly_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_yearly_date(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2020, Month::December, 24).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_yearly_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_day_forbidden_on_disabled_unique_date(c: &mut Criterion) {
    let start_date = Date::from_calendar_date(2020, Month::December, 8).unwrap();
    let config = create_config();
    c.bench_function("is_day_forbidden_on_disabled_unique_date", |b| {
        b.iter(|| config.is_day_forbidden(black_box(&start_date)))
    });
}

#[allow(dead_code)]
fn is_year_forbidden_in_disabled_year(c: &mut Criterion) {
    let config = create_config();
    c.bench_function("is_year_forbidden_in_disabled_year", |b| {
        b.iter(|| config.is_year_forbidden(black_box(2021)))
    });
}
