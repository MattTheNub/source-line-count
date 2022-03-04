use crate::count::count;

#[test]
fn basic() {
	let code = r#"<p>
	Hello,

	<span style="color: red">
		world!
	</span>
</p>"#;

	assert_eq!(count(code, "html").map(|x| x.lines), Some(6));
}

#[test]
fn comments() {
	let code = r#"<p>
	Hello,

<!-- this is a comment -->
	<span style="color: red">
		world!
	</span>
</p>"#;

	assert_eq!(count(code, "html").map(|x| x.lines), Some(6));
}
