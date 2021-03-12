extern crate minigrep;

// std::env::argsでコマンドライン引き数を読み取れるようにする
use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    // コマンドライン引数をコレクションにしてargs変数にぶち込む
    let args: Vec<String> = env::args().collect();
    // env::argsはminigrepに与えられたコマンドライン引き数のイテレータを返す
    // collectはコレクションを返す、あと変数に格納するときは型を明示する必要あり

    // argsのなかみを借用して、Configインスタンスを生成
    // unwrap_or_else以下でエラー時の処理を記述
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("引き数解析時に問題が発生しました:{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("アプリケーションエラーが発生しました:{}", e);
        process::exit(1);
    };
}
