use clap::Parser;
use rcal::main_lib;

fn main() {
    // コマンドライン引数パース
    let cli = rcal::cli::Cli::parse();
    // コンフィグ作成
    let mut config = rcal::config::Config::build(&cli);
    let executable = std::env::current_exe().expect("cannot determine executable path");
    config.holidays =
        match rcal::holiday::HolidayCalendar::load(&executable, cli.holiday_file.as_deref()) {
            Ok(holidays) => holidays,
            Err(error) => {
                eprintln!("failed to load holiday calendar: {error}");
                std::process::exit(1);
            }
        };

    // カレンダー文字列作成
    let lines = main_lib::exec(&config);

    // カレンダー表示
    for line in lines.iter() {
        println!("{}", line);
    }
}
