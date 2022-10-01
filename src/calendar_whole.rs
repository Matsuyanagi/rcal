use itertools;

use super::config;
use super::month_calendar;

pub struct CalendarWhole {}

impl CalendarWhole {
    pub fn new() -> Self {
        CalendarWhole {}
    }
    pub fn exec(config: &config::Config) {
        let multi_monthes = Self::create_month_calendar_vector(&config);

        /*         // テスト表示
               for calendar in multi_monthes.iter() {
                   println!("{}", calendar.temporal_to_string());
               }
               println!("----");
        */
        let calendar_lines = Self::format_month_calendar(&config, &multi_monthes);
        for line in calendar_lines.iter() {
            println!("{}", line);
        }
    }

    // 必要なだけ複数の月のカレンダーを Vec で返す
    fn create_month_calendar_vector(config: &config::Config) -> Vec<month_calendar::MonthCalendar> {
        // カレンダー取り扱い開始月
        let month_num_half: i32 = config.month_num as i32 / 2;

        // カレンダー取り扱い終了月
        let mut end_year: i32 = config.year as i32;
        let mut end_month: i32 = config.month as i32 - 1; // end_month : 0-11
        end_month += month_num_half;
        while end_month > 11 {
            end_year += 1;
            end_month -= 12;
        }
        end_month += 1; // end_month : 1-12

        let mut start_year: i32 = end_year;
        let mut start_month: i32 = end_month - 1; // start_month : 0-11
        if config.month_num > 1 {
            start_month -= config.month_num as i32 - 1;
            while start_month < 0 {
                start_year -= 1;
                start_month += 12;
            }
        }
        start_month += 1; // start_month : 1-12

        let mut monthes = Vec::with_capacity(config.month_num as usize);
        let today = chrono::Local::now().date_naive();

        // 各月カレンダー作成
        let mut m = start_month;
        let mut y = start_year;

        let mut cnt = 0;
        while cnt < config.month_num {
            let calendar = month_calendar::MonthCalendar::new(y as u32, m as u32, today);

            monthes.push(calendar);
            if y == end_year && m == end_month {
                break;
            }
            m += 1;
            if m == 13 {
                y += 1;
                m = 1;
            }
            cnt += 1;
        }

        monthes
    }

    // multi_monthes の月カレンダーベクタを config に従って表示できる Vec<String> に変換する
    pub fn format_month_calendar<'a>(
        config: &config::Config,
        multi_monthes: &'a Vec<month_calendar::MonthCalendar>,
    ) -> Vec<String> {
        // カレンダー文字列。ヘッダー込み
        let mut calendar_strs =
            Vec::with_capacity(8 * (config.month_num / config.calendar_month_column + 1) as usize);

        // 1行あたり calendar_month_column 月。
        //  ただし、途中で行の途中でカレンダーが終わるかもしれない。(4ヶ月表示するはずが2ヶ月しかない)
        let mut month_index = 0;

        loop {
            let month_end_index = std::cmp::min(
                month_index + config.calendar_month_column - 1,
                multi_monthes.len() as u32 - 1,
            );

            let row_monthes = &multi_monthes[month_index as usize..=month_end_index as usize];

            let mut monthes_strs = Self::conbine_month_to_strings(config, row_monthes);

            calendar_strs.append(&mut monthes_strs);
            if month_end_index >= multi_monthes.len() as u32 - 1 {
                break;
            }
            month_index += config.calendar_month_column;
        }

        calendar_strs
    }

    // 任意数の月カレンダーを水平方向に連結する
    fn conbine_month_to_strings(
        config: &config::Config,
        monthes: &[month_calendar::MonthCalendar],
    ) -> Vec<String> {
        let mut calendar_lines: Vec<String> = Vec::with_capacity(8);
        let months_len = monthes.len();

        // 月ごとのカレンダーの最大行数を求める。ヘッダ2行を含む
        let mut max_row_count = 0;
        for m in monthes.iter() {
            if max_row_count < m.calendar_weeks.len() {
                max_row_count = m.calendar_weeks.len();
            }
        }

        // ヘッダ
        let mut line_arr = Vec::with_capacity(max_row_count + 2);
        for m in monthes.iter() {
            line_arr.push(&m.header_year_month);
        }
        let line = itertools::join(&line_arr, config.month_border.as_str());
        calendar_lines.push(line);

        line_arr.clear();
        for m in monthes.iter() {
            line_arr.push(&m.header_day_of_week);
        }
        let line = itertools::join(&line_arr, config.month_border.as_str());
        calendar_lines.push(line);

        // 日付 : 月によって行数が変わる。unwrap_or("   ".repeat(7))
        // 全部の月が None (行がない) ならその行はくわえない
        let mut cal_line = Vec::with_capacity(months_len);
        let empty_line = "   ".repeat(7);
        for week_index in 0..max_row_count {
            cal_line.clear();
            for month in monthes {
                cal_line.push(month.calendar_weeks.get(week_index).unwrap_or(&empty_line));
            }
            let line = itertools::join(&cal_line, config.month_border.as_str());
            calendar_lines.push(line);
        }

        calendar_lines
    }
}
