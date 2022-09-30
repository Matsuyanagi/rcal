use super::config;
use super::month_calendar;

pub struct CalendarWhole {}

impl CalendarWhole {
    pub fn new() -> Self {
        CalendarWhole {}
    }
    pub fn exec(config: &config::Config) {
        let multi_monthes = Self::multi_month(&config);

        for calendar in multi_monthes {
            println!("{}", calendar.temporal_to_string());
        }
    }

    // 必要なだけ複数の月のカレンダーを Vec で返す
    fn multi_month(config: &config::Config) -> Vec<month_calendar::MonthCalendar> {
        let mut monthes = Vec::with_capacity(config.month_num as usize);

        let today = chrono::Local::now().date_naive();

        // カレンダー取り扱い開始月
        let minus_month_index: i32 = config.month_num as i32 / 2;
        let mut start_year: i32 = config.year as i32;
        let mut start_month: i32 = config.month as i32 - 1; // start_month : 0-11
        start_month -= minus_month_index;
        while start_month < 0 {
            start_year -= 1;
            start_month += 12;
        }
        start_month += 1; // start_month : 1-12

        // カレンダー取り扱い終了月
        let mut end_year: i32 = config.year as i32;
        let mut end_month: i32 = config.month as i32 - 1; // end_month : 0-11
        end_month += config.month_num as i32 - minus_month_index;
        while end_month > 11 {
            end_year += 1;
            end_month -= 12;
        }
        end_month += 1; // end_month : 1-12

        // 各月カレンダー作成
        let mut m = start_month;
        // for year in start_year..=end_year {
        let mut y = start_year;

        let mut cnt = 0;
        while cnt < config.month_num {
            // デバッグの保険のために最大月数を設定しておく
            let calendar = month_calendar::MonthCalendar::new(y as u32, m as u32, today);

            monthes.push(calendar);
            m += 1;
            if m == 13 {
                y += 1;
                m = 1;
            }
            if y == end_year && m == end_month {
                break;
            }
            cnt += 1;
        }

        monthes
    }
}
