use super::config;
use chrono::naive::NaiveDate;
use chrono::Datelike;
use chrono_utilities::naive::DateTransitions;
use colored::Colorize;

pub struct MonthCalendar {
    pub year: u32,
    pub month: u32,
    pub first_day: NaiveDate,
    pub first_day_of_week: i32,
    pub last_day: i32,
    pub is_today_month: bool,

    #[allow(dead_code)]
    today_year: u32,
    #[allow(dead_code)]
    today_month: u32,
    today_day: i32,

    pub header_year_month: String,
    pub header_day_of_week: String,
    pub calendar_weeks: Vec<String>,
}

impl MonthCalendar {
    pub fn new(config: &config::Config, year: u32, month: u32, today: &NaiveDate) -> Self {
        let first_day = chrono::NaiveDate::from_ymd(year as i32, month, 1);
        let last_day = first_day.end_of_month().unwrap().day() as i32;

        let mut first_day_of_week = first_day.weekday().number_from_monday() as i32;
        if first_day_of_week >= 7 {
            first_day_of_week -= 7; // Sun = 0, Mon = 1, ... Sat = 6
        }

        // ひと月のカレンダー作成パラメータ設定
        let mut month_calendar = MonthCalendar {
            year,
            month,
            first_day,
            first_day_of_week,
            last_day,
            is_today_month: first_day.year() == today.year() && first_day.month() == today.month(),
            today_year: today.year() as u32,
            today_month: today.month(),
            today_day: today.day() as i32,
            header_year_month: first_day.format(" %Y - %m            ").to_string(),
            header_day_of_week: String::from(" Su Mo Tu We Th Fr Sa "),
            calendar_weeks: Vec::with_capacity(8),
        };
        // カレンダーの日付部分行作成
        month_calendar.calendar_weeks = month_calendar.create_day_table(config);
        month_calendar
    }

    // year, month, first_day_of_week などから日付のならんだ calendar_weeks 表を作る。
    //  色を入れられるようにしておく
    fn create_day_table(&self, config: &config::Config) -> Vec<String> {
        let mut calendar_weeks = Vec::with_capacity(8);
        let day_space = "   ";
        const WEEK_STR_LENGTH: usize = 100; // 1日あたりの文字数と月の境ための隙間 1文字分。エスケープシーケンス分も追加。

        let minus_start_day = 1 - self.first_day_of_week;
        let mut week_str = String::with_capacity(WEEK_STR_LENGTH);

        let mut day_of_week = 0; // 曜日
        let mut today_pre_padding: colored::ColoredString;
        let mut today_post_padding = " ".normal();
        let mut flag_need_today_post_padding = false;
        for d in minus_start_day..=self.last_day {
            if d <= 0 {
                week_str.push_str(day_space);
            } else {
                // 今日の日付数字
                let mut number_str = format!("{:2}", d).normal();
                if config.colorize {
                    // 曜日によって色をつける
                    number_str = match day_of_week {
                        0 => number_str.bright_red(),  // 日曜
                        6 => number_str.bright_blue(), // 土曜
                        _ => number_str.normal(),      // 平日
                    };
                }
                if self.is_today_month && d == self.today_day {
                    // 本日
                    today_pre_padding = "[".normal();
                    today_post_padding = "]".normal();
                    flag_need_today_post_padding = true;
                } else {
                    // 本日以外
                    if flag_need_today_post_padding {
                        if day_of_week == 0 {
                            today_pre_padding = " ".normal();
                            today_post_padding = " ".normal();
                        } else {
                            today_pre_padding = "]".normal();
                            today_post_padding = " ".normal();
                        }
                        flag_need_today_post_padding = false;
                    } else {
                        today_pre_padding = " ".normal();
                    }
                }
                if config.colorize{
                    if self.is_today_month && d == self.today_day {
                        number_str = number_str.reverse();
                    }
                    today_pre_padding = today_pre_padding.yellow();
                    today_post_padding = today_post_padding.yellow();
                }
                

                let day_str = format!("{}{}", today_pre_padding, number_str);

                week_str.push_str(&day_str);
            }
            day_of_week += 1;
            if day_of_week == 7 {
                week_str.push_str(&today_post_padding.to_string());
                calendar_weeks.push(week_str);

                week_str = String::with_capacity(WEEK_STR_LENGTH);
                day_of_week = 0;
            }
        }
        if week_str.len() > 1 {
            week_str.push_str(today_post_padding.to_string().as_str());
            for _ in 0..7 - day_of_week {
                week_str.push_str(day_space);
            }
            calendar_weeks.push(week_str);
        }
        calendar_weeks
    }

    // デバグ用仮の表示用文字列出力
    pub fn temporal_to_string(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.header_year_month,
            self.header_day_of_week,
            self.calendar_weeks.join("\n")
        )
    }
}
