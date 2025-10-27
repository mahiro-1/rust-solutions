use clap::{App, Arg};

fn main() {
    //"{}"で次の引数を文字列に埋め込む。
    //{}のままだとオブジェクトがもつDisplayトレイトが呼び出される。定義されてなかったらエラー
    //{:?}だとDebugトレイトが呼び出されて、文字列が返ってくる。デバッグ用のやつだね
    //スペース区切りで引数を取得できる。一番初めの引数はプログラム自体のパスが渡される
    //println!("{:?}", std::env::args()); 

    //変数名に_を付けると変数が未使用であることをコンパイラなどに明示できる
    //let _matches = App::new("echor") //echorという名前で新しいAppを作成
    //    .version("0.1.0") //セマンティックバージョニングに従ったバージョンを記載
    //    .author("mahiro-1 < riceomeletomuraisu@gmail.com >") //作者名を設定
    //    .about("Rust echo") //プログラムの短い説明文を設定
    //    .get_matches(); //引数の解析をさせる
    let matches = App::new("echor")
        .version("0.1.0")
        .author("mahiro-1 <riceomeletomuraisu@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text") //引数が名前のArgを作って返す
                .value_name("TEXT") //helpで表示される値の型を書くことが出来る
                .help("Input text") //helpで表示されるこの引数の説明
                .required(true) //この引数が必須かどうかを設定する
                .min_values(1) //この引数の最小数を決める
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n") //この引数の短縮バージョンを設定
                .help("Do not print newline")
                .takes_value(false) //このオプションが引数を取るかどうか
        )
        .get_matches();
    //println!("{:#?}", matches); //#をつけることで整形して表示してくれる

    //Option<Vec<String>>を返す。
    //OptionはSome<T>またはNomeという値を値を持つ列挙型
    //Some<T>のときは内部にTという型の情報を持つ、Noneの時は値を持たない
    //Vec<T>は伸長可能な型Tの配列型
    //unwrapメソッドはSome<T>から値を取り出して返す。もしもOptionがNoneの値を持っていたらパニックする
    //今回の場合、textは必ず入力されるので問題なし
    let text = matches.values_of_lossy("text").unwrap(); 
    let omit_newline = matches.is_present("omit_newline"); //指定した引数が存在するかを真理値型で表して返す
    //{}の数だけ引数を指定して表示。Vec型のjoinメソッドで、要素をスペースで区切って文字列として出力。
    //Rustではifは式だから、値を返すことが出来る。この場合trueの時は""を返し、falseの時は"\n"を返す
    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}
