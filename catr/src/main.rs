fn main() {
    //letも式だから、定義した変数の値を返すもの。get_args()の返り値のResultの持つ関数を連続で呼び出している。
    //and_then()はResultがOkだった時に、引数に与えられた関数にResultの持つ値を渡す関数
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
