use crate::count::count;

#[test]
fn basic() {
	let code = r#"int sum(int a, int b) {
		return a + b;
	}"#;

	assert_eq!(count(code, "h").map(|x| x.lines), Some(3));
}
