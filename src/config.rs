use crate::cli;
use chrono::prelude::*;

pub struct Config {
    pub year: u32,
    pub month: u32,

    pub month_num: u32,
    pub calendar_month_column: u32,
    pub heuristic: bool,
    pub colorize: bool,
    pub month_border: String,
}

struct MY {
    first: Option<u32>,
    second: Option<u32>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            year: 0,
            month: 0,
            month_num: 0,
            calendar_month_column: 1,
            colorize: true,
            heuristic: false,
            month_border: "|".to_string(),
        }
    }

    pub fn from_year_month_num(year: u32, month: u32, month_num: u32) -> Self {
        Config {
            year,
            month,
            month_num,
            calendar_month_column: 1,
            colorize: true,
            heuristic: false,
            month_border: "|".to_string(),
        }
    }

    pub fn build(cli: &cli::Cli) -> Config {
        let mut config = Config::new();
        config.month_num = cli.month_num;
        config.colorize = !cli.nocolorize;
        config.heuristic = cli.heuristic;
        config.calendar_month_column = cli.calendar_month_column;

        let m = MY {
            first: cli.first,
            second: cli.second,
        };
        let datetime = Local::now();

        match m {
            MY { first, second }
                if (1u32..=12u32).contains(&first.unwrap_or_default())
                    && second.unwrap_or_default() >= 1900 =>
            {
                config.month = first.unwrap();
                config.year = second.unwrap();
                config
            }
            MY { first, second }
                if first.unwrap_or_default() >= 1900
                    && (1u32..=12u32).contains(&second.unwrap_or_default()) =>
            {
                config.month = second.unwrap();
                config.year = first.unwrap();
                config
            }
            MY {
                first,
                second: None,
            } if first.unwrap_or_default() >= 1900 => {
                config.month = datetime.month();
                config.year = first.unwrap();
                config
            }
            MY {
                first,
                second: None,
            } if (1u32..=12u32).contains(&first.unwrap_or_default()) => {
                config.month = first.unwrap();
                config.year = datetime.year() as u32;
                config
            }
            MY {
                first: None,
                second,
            } if (1u32..=12u32).contains(&second.unwrap_or_default()) => {
                config.month = second.unwrap();
                config.year = datetime.year() as u32;
                config
            }
            MY {
                first: None,
                second: None,
            } => {
                config.month = datetime.month();
                config.year = datetime.year() as u32;
                config
            }
            _ => panic!(
                "invalid first:{first}, second:{second}",
                first = m.first.unwrap_or_default(),
                second = m.second.unwrap_or_default()
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn config_test_01() {
        let cli = crate::cli::Cli {
            first: Some(2020),
            second: Some(1),
            month_num: 10,
            calendar_month_column: 2,
            heuristic: false,
            nocolorize: false,
        };

        let config = crate::config::Config::build(&cli);

        assert_eq!(config.year, 2020);
        assert_eq!(config.month, 1);
        assert_eq!(config.month_num, 10);
        assert_eq!(config.calendar_month_column, 2);
        assert_eq!(config.heuristic, false);
        assert_eq!(config.colorize, true);
    }

    #[test]
    fn config_test_02() {
        let cli = crate::cli::Cli {
            first: Some(2),
            second: Some(2000),
            month_num: 10,
            calendar_month_column: 2,
            heuristic: false,
            nocolorize: true,
        };

        let config = crate::config::Config::build(&cli);

        assert_eq!(config.year, 2000);
        assert_eq!(config.month, 2);
        assert_eq!(config.month_num, 10);
        assert_eq!(config.calendar_month_column, 2);
        assert_eq!(config.heuristic, false);
        assert_eq!(config.colorize, false);
    }
}
