#[test]
fn exits_0() {
    let result = crate::tests::std_builtin("pwd", Vec::new());
    assert_eq!(result.exit_code, 0);
}

#[test]
fn stdout_curdir() {
    let curdir = std::env::current_dir().unwrap();

    let result = crate::tests::std_builtin("pwd", Vec::new());
    assert_eq!(
        result.stdout.trim(),
        curdir.canonicalize().unwrap().to_str().unwrap()
    );
}
