use assert_cmd::Command;

#[test]
fn happy_path() {
    let mut cmd = Command::cargo_bin("icc").expect("Binary not found");
    cmd.arg("3,0,4,0,3,0,4,0,99");
    cmd.arg("123,234");

    cmd.assert().success().stdout("123,234\n").stderr("");
}