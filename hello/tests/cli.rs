use assert_cmd::Command;

#[test]
fn works(){
    //let mut cmd = Command::new("hello"); //helloコマンドを実行できるやつ
    //let res = cmd.output(); //実際に実行して、結果をresに保存
    //assert!(res.is_ok()); //assert!マクロ。中のブール式がtrueかどうか判定する
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}