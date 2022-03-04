use crate::count::count;

#[test]
fn basic() {
	let code = r#"fn main() {
			println!("Hello, world!");
		}"#;

	assert_eq!(count(code, "rs").map(|x| x.lines), Some(3));
}

#[test]
fn comments() {
	let code = r#"fn main() {
			// This is a comment
			println!("Hello, world!");
		}"#;

	assert_eq!(count(code, "rs").map(|x| x.lines), Some(3));
}

#[test]
fn multiline_comments() {
	let code = r#"fn main() {
			let string = "This is a very long string
/* with a fake comment inside of it */
";
		/* 
		this is a multiline
		comment
		*/

		println!("Hello, world!" /* this comment is in a line of code */);
		}"#;

	assert_eq!(count(code, "rs").map(|x| x.lines), Some(6));
}

#[test]
fn empty_lines() {
	let code = r#"fn main() {
			println!("Below is an empty line");

			println!("The next line only contains whitespace, so it's still empty");
		
		}"#;

	assert_eq!(count(code, "rs").map(|x| x.lines), Some(4));
}

#[test]
fn lone_quote() {
	let code = r#"fn main() {
			println!("The line below is not empty
"
);
		}"#;

	assert_eq!(count(code, "rs").map(|x| x.lines), Some(5));
}
