use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Cli {
    #[clap(value_parser, value_name = "MONTH", help="Month number : 1-12")]
    pub first: Option<u32>,
    #[clap(value_parser, value_name = "YEAR", help="Year : 1900-")]
    pub second: Option<u32>,

    #[clap(
        short = 'n',
        long = "num",
        value_name = "MONTH_NUM",
        help="Number of months to display.",
        value_parser,
        default_value_t = 3,
    )]
    pub month_num: u32,

    #[clap(
        short = 'c',
        long = "column",
        value_name = "MONTH_COLUMN",
        help="The number of calendar columns.",
        value_parser,
        default_value_t = 3
    )]
    pub calendar_month_column: u32,

    #[clap(short = 'z', long = "nocolor", value_name = "NO COLORIZE", action = clap::ArgAction::SetTrue, next_line_help = true, long_help = "No colorize.")]
    pub nocolorize: bool,
}
