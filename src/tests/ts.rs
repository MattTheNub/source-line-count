use crate::count::count;

#[test]
fn basic() {
	let code = r#"
let text: string = 'Hello, world!'

console.log(text)"#;

	assert_eq!(count(code, "ts").map(|x| x.lines), Some(2));
}
