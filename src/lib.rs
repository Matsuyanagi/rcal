#[allow(unused_imports)]
use chrono::prelude::*;
use chrono::offset::Local;
#[allow(unused_imports)]
use clap::{ArgEnum, Parser, Subcommand};
pub mod cli;
pub mod config;
pub mod month_calendar;

pub mod main_lib {
    use crate::month_calendar;


    pub fn exec(config: &mut crate::config::Config) {
        println!("Hello , rcal.");

        //    config.fix_year_month();

        println!("month : {:?}", config.month);
        println!("year : {:?}", config.year);
        println!("month_num : {:?}", config.month_num);
        println!("heuristic : {:?}", config.heuristic);
        
        let today = chrono::Local::now().date_naive();
        let calendar = month_calendar::MonthCalendar::new( config.year, config.month, today );
        println!("{}", calendar.temporal_to_string() );
        
    }
}

pub fn add_one(left: usize) -> usize {
    left + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
