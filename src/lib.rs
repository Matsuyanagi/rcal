use chrono::prelude::*;
#[allow(unused_imports)]
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(value_parser, default_value_t = Local::now().month())]
    pub month: u32,
    #[clap(value_parser, default_value_t = Local::now().year() as u32)]
    pub year: u32,

    #[clap(
        short = 'n',
        long = "num",
        value_name = "MONTH_NUM",
        value_parser,
        default_value_t = 3
    )]
    pub month_num: u32,

    #[clap(short, long, value_name = "HEAURISTIC", action = clap::ArgAction::SetTrue, next_line_help = true, long_help = "In January or December, the calendar for the previous year or the following year will be displayed.")]
    pub heuristic: bool,
}

pub fn exec(cli: &mut Cli) {
    println!("Hello , rcal.");

    let date_time = Local::now();


    println!("month : {:?}", cli.month);
    println!("year : {:?}", cli.year);
    println!("month_num : {:?}", cli.month_num);
    println!("heuristic : {:?}", cli.heuristic);
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
