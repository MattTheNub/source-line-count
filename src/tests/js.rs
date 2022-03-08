use crate::count::count;

#[test]
fn basic() {
	let code = r#"console.log('Hello')
	console.log('World!')"#;

	assert_eq!(count(code, "js").map(|x| x.lines), Some(2));
}

#[test]
fn comments() {
	let code = r#"console.log('Hello')
	console.log('World!')
	// this is a comment"#;

	assert_eq!(count(code, "js").map(|x| x.lines), Some(2));
}

#[test]
fn multiline_comments() {
	let code = r#"console.log('Hello')
	console.log(`/*
		not a comment
	*/`)
	/*
	this is a comment
	*/
	"#;

	assert_eq!(count(code, "js").map(|x| x.lines), Some(4));
}
