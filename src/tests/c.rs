use crate::count::count;

#[test]
fn basic() {
	let code = r#"int main() {
		 printf("Hello, world!");
	}"#;

	assert_eq!(count(code, "c").map(|x| x.lines), Some(3));
}
