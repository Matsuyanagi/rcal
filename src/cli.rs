use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(value_parser, value_name = "MONTH")]
    pub first: Option<u32>,
    #[clap(value_parser, value_name = "YEAR")]
    pub second: Option<u32>,

    #[clap(
        short = 'n',
        long = "num",
        value_name = "MONTH_NUM",
        value_parser,
        default_value_t = 3
    )]
    pub month_num: u32,

    #[clap(
        short = 'c',
        long = "column",
        value_name = "MONTH_COLUMN",
        value_parser,
        default_value_t = 3
    )]
    pub calendar_month_column: u32,

    #[clap(short = 'z', long = "nocolor", value_name = "NO COLORIZE", action = clap::ArgAction::SetTrue, next_line_help = true, long_help = "No colorize.")]
    pub nocolorize: bool,
}
