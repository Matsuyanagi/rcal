#[allow(unused_imports)]
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    // #[clap(parse(from_os_str))]
    #[clap(short, long, value_name = "YEAR", value_parser)]
    pub year: Option<u32>,
    #[clap(short, long, value_name = "MONTH", value_parser)]
    pub month: Option<u32>,

    #[clap(short = 'n', long = "num", value_name = "MONTH_NUM", value_parser)]
    pub month_num: Option<u32>,

    #[clap(short, long, value_name = "HEAURISTIC")]
    pub heuristic: bool,
}

pub fn exec(cli: &Cli) {
    println!("Hello , rcal.");
    if let Some(year) = cli.year {
        println!("year : {:?}", year);
    }
    if let Some(month) = cli.month {
        println!("month : {:?}", month);
    }
    if let Some(month_num) = cli.month_num {
        println!("month_num : {:?}", month_num);
    }
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
