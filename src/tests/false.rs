use crate::tests::std_builtin;

#[test]
fn test() {
    let result = std_builtin("false", Vec::new());
    assert_eq!(result, 1);
}
