use chrono::naive::NaiveDate;
use chrono::{Datelike, Local, Months};
use chrono_utilities::naive::DateTransitions;

pub struct MonthCalendar {
    pub year: u32,
    pub month: u32,
    pub first_day: NaiveDate,
    // pub first_day_of_week: chrono::Weekday,
    pub first_day_of_week: i32,
    pub last_day: i32,
    pub is_today_month: bool,

    today_year: u32,
    today_month: u32,
    today_day: i32,

    pub header_year_month: String,
    pub header_day_of_week: String,
    pub calendar_weeks: Vec<String>,
}

impl MonthCalendar {
    pub fn new(year: u32, month: u32, today: NaiveDate) -> Self {
        let first_day = chrono::NaiveDate::from_ymd(year as i32, month, 1);
        let last_day = first_day.end_of_month().unwrap().day() as i32;

        let mut month_calendar = MonthCalendar {
            year,
            month,
            first_day,
            first_day_of_week: first_day.weekday().number_from_monday() as i32 % 7,     // Sun = 0, Mon = 1, ... Sat = 6
            last_day,
            is_today_month: first_day.year() == today.year() && first_day.month() == today.month(),
            today_year: today.year() as u32,
            today_month: today.month(),
            today_day: today.day() as i32,
            header_year_month: first_day.format("%Y - %m").to_string(),
            header_day_of_week: String::from(" Su Mo Tu We Th Fr St"),
            calendar_weeks: Vec::with_capacity(8),
        };
        // month_calendar.create_day_table();
        month_calendar.calendar_weeks = month_calendar.create_day_table();
        month_calendar
    }
    
    // year, month, first_day_of_week などから日付のならんだ calendar_weeks 表を作る。
    //  色を入れられるようにしておく
    fn create_day_table(&mut self) -> Vec<String>{
        // self.calendar_weeks.push( self.header_year_month.clone() );
        // self.calendar_weeks.push( self.header_day_of_week.clone() );
        let mut calendar_weeks = Vec::with_capacity(8);
        
        let minus_start_day = 1 - self.first_day_of_week;
        let mut week_str: String = "".to_string();
        
        let mut day_count = 0;
        for d in minus_start_day ..= self.last_day {
            if d <= 0{
                week_str += "   ";
            }else{
                if self.is_today_month && d == self.today_day{
                    week_str += format!(">{:2}",d).as_str();
                }else{
                    week_str += format!(" {:2}",d).as_str();
                }
            }
            day_count += 1;
            if day_count == 7{
                calendar_weeks.push(week_str);
                week_str = "".to_string();
                day_count = 0;
            }
        }
        if ! week_str.is_empty(){
            calendar_weeks.push(week_str);
        }
        calendar_weeks
    }
    
    
    pub fn temporal_to_string(&self) -> String {
        format!( "{}\n{}\n{}", self.header_year_month, self.header_day_of_week, self.calendar_weeks.join("\n") )
    }
    
    
}