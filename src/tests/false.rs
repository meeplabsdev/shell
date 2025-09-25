#[test]
fn exits_1() {
    let result = crate::tests::std_builtin("false", Vec::new());
    assert_eq!(result.exit_code, 1);
}
