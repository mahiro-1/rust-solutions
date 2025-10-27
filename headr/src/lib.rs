use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines [default: 10]")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Number of lines")
                .takes_value(true)
                .value_name("BYTES")
                .conflicts_with("lines")
        )
        .get_matches();
    /* 失敗作
    let lines_val = parse_positive_int(matches.value_of("lines").unwrap());
    let bytes_val = parse_positive_int(matches.value_of("bytes").unwrap());
    if(lines_val.is_err()){
        return Err(lines_val);
    }
    if(bytes_val.is_err()){
        return Err(bytes_val);
    }
    Ok(Config {
        files: ,
        lines: lines_val.unwrap(),
        bytes: bytes_val.unwrap(),
    })
    */
    let lines = matches
        .value_of("lines") //value_ofでOption<&str>を取得
        .map(parse_positive_int) //SomeだったものをOptionから取り出し、関数に流す
        .transpose() //Option<Result>をResult<Option>に変換する
        .map_err(|e| format!("illegal line count -- {}", e))?; //エラーなら中身のエラーメッセージを返し、OKならそのまま、?を使ってエラー伝播かOkをアンパックする

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes //もともとOption型だからそのまま書いてキーを省略する
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==", if file_num > 0 {"\n"} else {""}, filename);
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64); //指定したバイト数を読み込む、返すのはファイルハンドラ
                    let mut buffer = vec![0; num_bytes]; //0で初期化した長さnum_bytesのバッファーを用意する
                    let bytes_read = handle.read(&mut buffer)?; //引数にはポインタを渡して代入してもらう。返り値は実際に読み込めたバイト数
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read])); //実際に読み込まれたバイト数分だけ文字列に変換して書き出す
                }
                else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        };
    }
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}