use assert_cmd::Command;
use predicates::prelude::*;
use std::fs; //ファイルシステムモジュール

type TestResult = Result<(), Box<dyn std::error::Error>>;

//引数なしで失敗することを確かめるテスト関数。diesを入れて失敗するのを見ている関数であることを明記
//cargo test diesとすると関数名にdiesが入っているものだけを実行してくれる
#[test]
fn dies_no_args() -> TestResult {
    //let mut cmd = Command::cargo_bin("echor").unwrap();
    //cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt") //呼んだ関数が返す値がこの関数の返り値
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

