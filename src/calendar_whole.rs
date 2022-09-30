use super::month_calendar;
use super::config;

pub struct CalendarWhole {}

impl CalendarWhole {
    pub fn new() -> Self {
        CalendarWhole {}
    }
    pub fn exec( config: &config::Config ) {
        let today = chrono::Local::now().date_naive();
        let calendar = month_calendar::MonthCalendar::new(config.year, config.month, today);

        println!("{}", calendar.temporal_to_string());
    }
}
