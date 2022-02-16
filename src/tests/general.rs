use crate::count::count;

#[test]
fn unknown_extension() {
	assert_eq!(
		count("Hello, world!", "this_extension_does_not_exist"),
		None
	);
}
