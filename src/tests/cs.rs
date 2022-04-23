use crate::count::count;

#[test]
fn basic() {
	let code = r#"
using System;
namespace HelloWorld;

class HelloWorld
{
	static void Main(string[] args)
	{
		Console.WriteLine("Hello World!");
	}
}"#;

	assert_eq!(count(code, "cs").map(|x| x.lines), Some(9));
}

#[test]
fn verbatim_strings() {
	let code = r#"
using System;
namespace HelloWorld;

class HelloWorld
{
	static void Main(string[] args)
	{
		Console.WriteLine(
			@"This is a verbatim string\"
			// this is no longer a part of the string
		);
	}
}"#;

	assert_eq!(count(code, "cs").map(|x| x.lines), Some(11));
}
