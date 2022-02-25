use crate::count::count;

#[test]
fn basic() {
	let code = r#"int main() {
		cout << "Hello, world!" << endl;
	}"#;

	assert_eq!(count(code, "cpp"), Some(3));
}

#[test]
fn comments() {
	let code = r#"int main() {
		// This is a comment
		/*
		This is a multiline comment
		*/
		cout << "Hello, world!" << endl;
	}"#;

	assert_eq!(count(code, "cpp"), Some(3));
}

#[test]
fn raw_strings() {
	let code = r#"int main() {
		cout << R"(this is a raw string: " /*
this quote is ignored, and this is still not a comment */
// and neither is this)";
	}"#;

	assert_eq!(count(code, "cpp"), Some(5));
}
