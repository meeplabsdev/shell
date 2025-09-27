#[test]
fn exits_0() {
    let result = crate::tests::std_builtin("cd", Vec::new());
    assert_eq!(result.exit_code, 0);
}

#[test]
fn exits_0_dashp() {
    let result = crate::tests::std_builtin("cd", vec!["-p".into(), "/".into()]);
    assert_eq!(result.exit_code, 0);
}

#[test]
fn exits_0_prevdir() {
    let result = crate::tests::std_builtin("cd", vec!["^".into()]);
    assert_eq!(result.exit_code, 0);
}

#[test]
fn exits_1_noexist() {
    let result = crate::tests::std_builtin("cd", vec!["/thisdoesnotexist".into()]);
    assert_eq!(result.exit_code, 1);
}
