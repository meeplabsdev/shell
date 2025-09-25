#[test]
fn exits_0() {
    let result = crate::tests::std_builtin("true", Vec::new());
    assert_eq!(result.exit_code, 0);
}
