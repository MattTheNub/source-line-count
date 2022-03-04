use crate::count::count;

#[test]
fn unknown_extension() {
	assert!(count("Hello, world!", "this_extension_does_not_exist").is_none());
}
