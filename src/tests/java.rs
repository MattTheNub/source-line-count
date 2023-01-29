use crate::count::count;

#[test]
fn basic() {
	let code = r#"
public class HelloWorld {
	public static void main(String[] args) {
		System.out.println("Hello, world!");
	}
}"#;

	assert_eq!(count(code, "java").map(|x| x.lines), Some(5));
}

#[test]
fn raw_string() {
	let code = r#"
public class HelloWorld {
	public static void main(String[] args) {
		System.out.println(```A raw string ` with
			/* a fake comment */
		```);
	}
}"#;

	assert_eq!(count(code, "java").map(|x| x.lines), Some(7));
}
