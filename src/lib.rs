pub mod calendar_whole;
pub mod cli;
pub mod config;
pub mod month_calendar;

pub mod main_lib {
    use crate::calendar_whole;

    pub fn exec(config: &crate::config::Config) {
        calendar_whole::CalendarWhole::exec(&config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let config = crate::config::Config {
            year: 2022,
            month: 1,
            month_num: 1,
            calendar_month_column: 3,
            heuristic: false,
            month_border: " |".to_string(),
        };
        let today_day = chrono::NaiveDate::from_ymd(2000, 1, 1);
        let calendar = month_calendar::MonthCalendar::new(config.year, config.month, today_day);

        let expect_answer = r#" 2022 - 01           
 Su Mo Tu We Th Fr Sa
                    1
  2  3  4  5  6  7  8
  9 10 11 12 13 14 15
 16 17 18 19 20 21 22
 23 24 25 26 27 28 29
 30 31               "#;
        assert_eq!(calendar.temporal_to_string(), expect_answer);
    }
    #[test]
    fn test02_leap_year() {
        let config = crate::config::Config {
            year: 2015,
            month: 2,
            month_num: 1,
            calendar_month_column: 3,
            heuristic: false,
            month_border: " |".to_string(),
        };
        let today_day = chrono::NaiveDate::from_ymd(2000, 1, 1);
        let calendar = month_calendar::MonthCalendar::new(config.year, config.month, today_day);

        let expect_answer = r#" 2015 - 02           
 Su Mo Tu We Th Fr Sa
  1  2  3  4  5  6  7
  8  9 10 11 12 13 14
 15 16 17 18 19 20 21
 22 23 24 25 26 27 28"#;
        assert_eq!(calendar.temporal_to_string(), expect_answer);
    }
}
