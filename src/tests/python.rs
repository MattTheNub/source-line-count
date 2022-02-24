use crate::count::count;

#[test]
fn basic() {
	let code = r#"print('Hello')
	print('World!')"#;

	assert_eq!(count(code, "py"), Some(2));
}

#[test]
fn comments() {
	let code = r#"print('Hello')
	print('World!')
	# This is a comment"#;

	assert_eq!(count(code, "py"), Some(2));
}

#[test]
fn lone_quote() {
	let code = r#"print('''Hello
'''
)
	print('World!')"#;

	assert_eq!(count(code, "py"), Some(4));
}
