#[allow(unused_imports)]
use chrono::prelude::*;
#[allow(unused_imports)]
use clap::{ArgEnum, Parser, Subcommand};
pub mod cli;
pub mod config;

pub mod main_lib {

    pub fn exec(config: &mut crate::config::Config) {
        println!("Hello , rcal.");

        //    config.fix_year_month();

        println!("month : {:?}", config.month);
        println!("year : {:?}", config.year);
        println!("month_num : {:?}", config.month_num);
        println!("heuristic : {:?}", config.heuristic);
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
