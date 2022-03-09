use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum StringMode {
	Normal {
		quotes: &'static [&'static str],
		escape: bool,
	},
	Rust,
	Cxx,
}

pub struct LangInfo {
	pub name: &'static str,
	pub single_line_comment: Option<&'static str>,
	pub start_comment: Option<&'static str>,
	pub end_comment: Option<&'static str>,
	pub string_mode: StringMode,
}

macro_rules! langs {
	($($($key:literal)|+ => $val:expr),*) => {{
		#[allow(unused_mut)]
		let mut map = ::std::collections::HashMap::new();

		$(
			$(
				map.insert($key.to_owned(), $val);
			)+
		)*
		map
	}};
}

lazy_static! {
	pub static ref LANGS: HashMap<String, LangInfo> = langs! {
		"rs" => LangInfo {
			name: "Rust",
			single_line_comment: Some("//"),
			start_comment: Some("/*"),
			end_comment: Some("*/"),
			string_mode: StringMode::Rust,
		},
		"py" | "pyi" | "pyw" => LangInfo {
			name: "Python",
			single_line_comment: Some("#"),
			start_comment: None,
			end_comment: None,
			// triple quotes need priority over normal quotes
			string_mode: StringMode::Normal {
				quotes: &[r#"""""#, "'''", r#"""#, "'"],
				escape: true,
			}
		},
		"cpp" | "cxx" | "c++" | "cc" | "h" | "hh" | "hpp" | "hxx" | "h++" => LangInfo {
			name: "C++",
			single_line_comment: Some("//"),
			start_comment: Some("/*"),
			end_comment: Some("*/"),
			string_mode: StringMode::Cxx,
		},
		"html" | "htm" => LangInfo {
			name: "HTML",
			single_line_comment: None,
			start_comment: Some("<!--"),
			end_comment: Some("-->"),
			string_mode: StringMode::Normal {
				quotes: &[r#"""#, "'"],
				escape: false,
			}
		},
		"js" | "cjs" | "mjs" | "jsx" => LangInfo {
			name: "JavaScript",
			single_line_comment: Some("//"),
			start_comment: Some("/*"),
			end_comment: Some("*/"),
			string_mode: StringMode::Normal {
				quotes: &["`", r#"""#, "'"],
				escape: true,
			}
		},
		"ts" | "tsx" => LangInfo {
			name: "TypeScript",
			single_line_comment: Some("//"),
			start_comment: Some("/*"),
			end_comment: Some("*/"),
			string_mode: StringMode::Normal {
				quotes: &["`", r#"""#, "'"],
				escape: true,
			}
		}
	};
}
