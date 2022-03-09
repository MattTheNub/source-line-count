use crate::count::count;

#[test]
fn basic() {
	let code = r#"
body {
	margin: 0;
	padding: 0;
}"#;

	assert_eq!(count(code, "css").map(|x| x.lines), Some(4));
}

#[test]
fn comments() {
	let code = r#"
body {
	margin: 0;
	/*
	this is a
	CSS comment
	*/
	padding: 0;
}"#;

	assert_eq!(count(code, "css").map(|x| x.lines), Some(4));
}

#[test]
fn strings() {
	let code = r#"
body {
	font-family: '/*foo';
}
.bar {
	font-family: 'baz*/';
}
"#;

	assert_eq!(count(code, "css").map(|x| x.lines), Some(6));
}
