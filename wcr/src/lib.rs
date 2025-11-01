use clap::{App, Arg};
use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn print_wc(fileinfo: FileInfo, config: Config, filename: &str){
    if config.lines { print!("{:8}", fileinfo.num_lines) }
    if config.words { print!("{:8}", fileinfo.num_words) }
    if config.bytes { print!("{:8}", fileinfo.num_bytes) }
    if config.chars { print!("{:8}", fileinfo.num_chars) }
    if filename != "-" {
        println!(" {}", filename)
    }
    else {
        println!("");
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_info = FileInfo {num_lines: 0, num_words: 0, num_bytes: 0, num_chars: 0};
    for filename in &config.files.clone() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let fileinfo = count(file)?;
                print_wc(fileinfo.clone(), config.clone(), filename);
                total_info.num_lines += fileinfo.num_lines;
                total_info.num_words += fileinfo.num_words;
                total_info.num_bytes += fileinfo.num_bytes;
                total_info.num_chars += fileinfo.num_chars;
            }
        }
    }
    if config.files.len() > 1 {
        print_wc(total_info, config, "total");
    }
    Ok(())
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> { //BufReadトレイトを実装している引数を取る
    let mut num_lines = 0; //read_lineしてOk(o)を返すまでループして、そのループ数が行数
    let mut num_words = 0; //これが一番だるい、スペースやタブ、改行などで区切られた文字をカウントする。複数スペースやタブとスペース連続なども考えなきゃならん -> .split_whitespace()で解決かも
    let mut num_bytes = 0; //read_lineの返り値がその行のバイト数だからそれを足すだけ
    let mut num_chars = 0; //これもだるい、一つ一つの文字を読み取ってカウントしていく...
    
    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes;
        num_words += line.split_whitespace().count();
        //println!("{:#?}", line.split_whitespace());
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo { 
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("mahiro-1")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .get_matches();
    let mut lines = matches.is_present("lines");
    let mut words =  matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    if [lines, words, bytes, chars].iter().all(|v| v == &false){
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)] //テストの時だけコンパイルされる
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}