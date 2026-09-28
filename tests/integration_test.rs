// use rcal;
mod common;

use chrono::NaiveDate;
use clap::Parser;
use rcal::{cli::Cli, config::Config, holiday::HolidayCalendar, month_calendar::MonthCalendar};
use std::path::Path;

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[test]
fn holiday_rule_handles_recurrence_exclusions_and_folded_lines() {
    let ics = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART;VALUE=DATE:20200113\r\nRRULE:FREQ=YEARLY;BYMONTH=1;BYDAY=2MO;UNTIL=20271231\r\nEXDATE;VALUE=DATE:20210111,\r\n 20230109\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
    let holidays = HolidayCalendar::from_ics(ics).unwrap();
    assert!(holidays.contains(date(2020, 1, 13)));
    assert!(!holidays.contains(date(2021, 1, 11)));
    assert!(holidays.contains(date(2022, 1, 10)));
    assert!(!holidays.contains(date(2023, 1, 9)));
    assert!(holidays.contains(date(2027, 1, 11)));
    assert!(!holidays.contains(date(2028, 1, 10)));
}

#[test]
fn holiday_rule_combines_month_day_and_weekday_filters() {
    let ics = "BEGIN:VEVENT\nDTSTART;VALUE=DATE:20200504\nRRULE:FREQ=YEARLY;BYMONTH=5;BYMONTHDAY=4;BYDAY=TU,WE,TH,FR,SA;\n UNTIL=20271231\nEND:VEVENT\n";
    let holidays = HolidayCalendar::from_ics(ics).unwrap();
    assert!(!holidays.contains(date(2026, 5, 4)));
    assert!(holidays.contains(date(2027, 5, 4)));
}

#[test]
fn all_day_event_covers_dates_before_exclusive_end_and_rdate() {
    let ics = "BEGIN:VEVENT\nDTSTART;VALUE=DATE:20260102\nDTEND;VALUE=DATE:20260104\nRDATE;VALUE=DATE:20260210\nEND:VEVENT\n";
    let holidays = HolidayCalendar::from_ics(ics).unwrap();
    assert!(holidays.contains(date(2026, 1, 2)));
    assert!(holidays.contains(date(2026, 1, 3)));
    assert!(!holidays.contains(date(2026, 1, 4)));
    assert!(holidays.contains(date(2026, 2, 10)));
    assert!(holidays.contains(date(2026, 2, 11)));
}

#[test]
fn unsupported_recurrence_rule_reports_an_error() {
    let ics = "BEGIN:VEVENT\nDTSTART;VALUE=DATE:20260102\nRRULE:FREQ=DAILY\nEND:VEVENT\n";
    assert!(HolidayCalendar::from_ics(ics).is_err());
}

#[test]
fn holiday_file_next_to_executable_and_explicit_override() {
    let folder = std::env::temp_dir().join(format!(
        "rcal-holiday-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&folder).unwrap();
    let exe = folder.join("rcal.exe");
    let default_file = folder.join("rcal.ics");
    let other_file = folder.join("other.ics");
    std::fs::write(
        &default_file,
        "BEGIN:VEVENT\nDTSTART;VALUE=DATE:20260102\nEND:VEVENT\n",
    )
    .unwrap();
    std::fs::write(
        &other_file,
        "BEGIN:VEVENT\nDTSTART;VALUE=DATE:20260103\nEND:VEVENT\n",
    )
    .unwrap();

    let automatic = HolidayCalendar::load(&exe, None).unwrap();
    assert!(automatic.contains(date(2026, 1, 2)));
    let explicit = HolidayCalendar::load(&exe, Some(Path::new(&other_file))).unwrap();
    assert!(!explicit.contains(date(2026, 1, 2)));
    assert!(explicit.contains(date(2026, 1, 3)));
    assert!(HolidayCalendar::load(&exe, Some(&folder.join("missing.ics"))).is_err());

    std::fs::remove_file(&default_file).unwrap();
    assert!(
        !HolidayCalendar::load(&exe, None)
            .unwrap()
            .contains(date(2026, 1, 2))
    );

    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn holiday_cli_option_is_available() {
    let cli = Cli::try_parse_from(["rcal", "2026", "1", "--holidays", "japan.ics"]).unwrap();
    assert_eq!(cli.holiday_file.as_deref(), Some(Path::new("japan.ics")));
}

#[test]
fn holiday_on_weekday_and_saturday_uses_holiday_color() {
    colored::control::set_override(true);
    let holidays = HolidayCalendar::from_ics("BEGIN:VEVENT\nDTSTART;VALUE=DATE:20260102\nEND:VEVENT\nBEGIN:VEVENT\nDTSTART;VALUE=DATE:20260103\nEND:VEVENT\n").unwrap();
    let mut config = Config::from_year_month_num(2026, 1, 1);
    config.holidays = holidays;
    let month = MonthCalendar::new(&config, 2026, 1, &date(2000, 1, 1));
    let first_week = &month.calendar_weeks[0];
    assert!(first_week.contains("\u{1b}[91m 2\u{1b}[0m"));
    assert!(first_week.contains("\u{1b}[91m 3\u{1b}[0m"));
    config.colorize = false;
    let plain = MonthCalendar::new(&config, 2026, 1, &date(2000, 1, 1));
    assert!(!plain.temporal_to_string().contains('\u{1b}'));
    colored::control::unset_override();
}

#[test]
fn test01() {
    common::setup();
    assert_eq!(1, 1);
}
