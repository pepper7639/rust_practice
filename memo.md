std::env::argsでコマンドライン引き数を読み取れるようにする  
env::argsはminigrepに与えられたコマンドライン引き数のイテレータを返す  
collectはコレクションを返す、あと変数に格納するときは型を明示注釈する必要あり  
expectは？でも可  

以下  
https://doc.rust-jp.rs/book-ja/ch12-03-improving-error-handling-and-modularity.html  
参照

## リファクタリング
    ・機能を小分けする:具体的には各関数が一つの機能を持つようにする
    ・設定用変数を一つの構造に押し込み、コードを明瞭化する
    ・エラー時のメッセージを正しく表示させる
    ・範囲外アクセス(indx out of bounds)エラー時の挙動を書く


## バイナリプロジェクトの責任の分離
    ・プログラムをmain.rsとlib.rsに分離
    ・lib.rsにはロジックを書く
    ・main.rsにはプログラムの実行を行う