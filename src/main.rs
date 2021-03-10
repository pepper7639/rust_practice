// std::env::argsでコマンドライン引き数を読み取れるようにする
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // env::argsはminigrepに与えられたコマンドライン引き数のイテレータを返す
    // collectはコレクションを返す、あと変数に格納するときは型を明示する必要あり
    let query = &args[1];
    let filename = &args[2];

    // {}を探しています
    println!("検索中:{}", query);
    // {}というファイルの中
    println!("該当ファイル:{}", filename);

    // ファイルが見つからない場合expect
    let mut f = File::open(filename).expect("ファイルが見つかりません。");

    let mut contents = String::new();

    // ファイルを読み込んで、書かれてる内容をcontentsにぶち込む
    f.read_to_string(&mut contents)
        .expect("ファイルの読み込み中に問題がありました。");

    // ファイルに書いてある文字を表示
    println!("テキスト内容:\n{}", contents);
}
