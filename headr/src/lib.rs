use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

//usize型はポインタサイズの符号なし整数型。つまり整数入れれる型。64bitだと8byte
//bytesはユーザーが有効な値を与えた場合はSome<usize>に、与えなかった場合はNoneになる
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("mahiro-1")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                
        )
        .get_matches();

    Ok(
        files: aaa,
        lines: aaa,
        bytes: aaa,
    )
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

//正の整数かを解析する関数（コンピュータサイエンス用語）
fn parse_positive_int(val: &str) -> MyResult<usize> {
    /* 失敗コード
    let res = val.parse::<usize>();
    if res.is_err() {
        return res;
    }
    let num = res.unwrap();
    if  0 >= num {
        return Err(());
    }
    Ok(res)
    */
    //matchアームのガードという機能でパターン分岐後の条件分岐を作成したらしい
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)), //Fromは型変換を支援するもの、今回だとエラートレイトがあるように変換してくれる
    }
}

//単体テスト用の関数
#[test]
fn test_parse_povitive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}