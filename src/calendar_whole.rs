use itertools;

use super::config;
use super::month_calendar;

pub struct CalendarWhole {}

impl CalendarWhole {
    pub fn new() -> Self {
        CalendarWhole {}
    }
    pub fn exec(config: &config::Config) -> Vec<String> {
        let today = chrono::Local::now().date_naive();
        let multi_monthes = Self::create_month_calendar_vector(&config, &today);

        let calendar_lines = Self::format_month_calendar(&config, &multi_monthes);

        calendar_lines
    }

    // 必要なだけ複数の月のカレンダーを Vec で返す
    pub fn create_month_calendar_vector(
        config: &config::Config,
        today: &chrono::NaiveDate,
    ) -> Vec<month_calendar::MonthCalendar> {
        // 開始年月、終了年月を求める
        let (start_year, start_month, end_year, end_month) = Self::start_end_month(&config);

        // 各月カレンダー配列
        let mut monthes = Vec::with_capacity(config.month_num as usize);

        // 各月カレンダー作成
        let mut m = start_month;
        let mut y = start_year;

        for _ in 0..config.month_num {
            let calendar = month_calendar::MonthCalendar::new(&config, y as u32, m as u32, &today);

            monthes.push(calendar);
            if y == end_year && m == end_month {
                break;
            }
            m += 1;
            if m == 13 {
                y += 1;
                m = 1;
            }
        }

        monthes
    }

    // 表示開始年月と終了年月を求める
    fn start_end_month(config: &config::Config) -> (i32, i32, i32, i32) {
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

        (start_year, start_month, end_year, end_month)
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
            // 次行の開始インデックスへ進める
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

        // 月ごとのカレンダーの最大行数を求める。
        let max_row_count = monthes.iter().fold(0usize, |max_length, month| {
            std::cmp::max(max_length, month.calendar_weeks.len())
        });

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
        let mut cal_line = Vec::with_capacity(monthes.len());
        let empty_line = "   ".repeat(7) + " ";
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

#[cfg(test)]
mod tests {

    #[test]
    fn monthes_lines_count() {
        let config = crate::config::Config::from_year_month_num(2022, 1, 3);

        let today = chrono::NaiveDate::from_ymd(2015, 2, 1);
        let multi_monthes =
            crate::calendar_whole::CalendarWhole::create_month_calendar_vector(&config, &today);

        assert_eq!(multi_monthes.len() as u32, config.month_num);

        // テスト表示
        let mut lines = Vec::new();
        for calendar in multi_monthes.iter() {
            let mut str = calendar.temporal_to_string();
            // 末尾が改行コードなら1文字削除。ruby の chomp 相当
            if *(str.as_bytes().last().unwrap()) == b'\n' {
                str.pop();
            }
            let str_collect = str.split('\n').collect::<Vec<&str>>();
            let vec_lines = str_collect.iter().map(|s| s.to_string());
            lines.extend(vec_lines);
        }
        println!("{:?}", lines);

        assert_eq!(
            lines.len(),
            multi_monthes
                .iter()
                .fold(0, |num, month| num + month.calendar_weeks.len() + 2)
        );
    }

    #[test]
    fn test_month_2015_02_leapyear() {
        let mut config = crate::config::Config::from_year_month_num(2015, 2, 3);
        let today = chrono::NaiveDate::from_ymd(2015, 2, 1);
        config.colorize = false;
        config.calendar_month_column = 3;

        let multi_monthes =
            crate::calendar_whole::CalendarWhole::create_month_calendar_vector(&config, &today);

        let calendar_lines =
            crate::calendar_whole::CalendarWhole::format_month_calendar(&config, &multi_monthes);
        let calendar_lines_str = calendar_lines.join("\n");

        let expect_answer = r#" 2015 - 01            | 2015 - 02            | 2015 - 03            
 Su Mo Tu We Th Fr Sa | Su Mo Tu We Th Fr Sa | Su Mo Tu We Th Fr Sa 
              1  2  3 |[ 1] 2  3  4  5  6  7 |  1  2  3  4  5  6  7 
  4  5  6  7  8  9 10 |  8  9 10 11 12 13 14 |  8  9 10 11 12 13 14 
 11 12 13 14 15 16 17 | 15 16 17 18 19 20 21 | 15 16 17 18 19 20 21 
 18 19 20 21 22 23 24 | 22 23 24 25 26 27 28 | 22 23 24 25 26 27 28 
 25 26 27 28 29 30 31 |                      | 29 30 31             "#;
        assert_eq!(calendar_lines_str, expect_answer);
    }
}
