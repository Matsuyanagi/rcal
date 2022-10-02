use clap::Parser;
use rcal::main_lib;

fn main() {
    // コマンドライン引数パース
    let cli = rcal::cli::Cli::parse();
    // コンフィグ作成
    let config = rcal::config::Config::build(&cli);

    // カレンダー文字列作成
    let lines = main_lib::exec(&config);

    // カレンダー表示
    for line in lines.iter() {
        println!("{}", line);
    }
}
