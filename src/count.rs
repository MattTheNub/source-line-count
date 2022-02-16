use crate::{
	lang::{StringMode, LANGS},
	util::{advance_up_to, peek_string},
};
use itertools::multipeek;

pub fn count(file: &str, ext: &str) -> Option<usize> {
	let mut chars = multipeek(file.chars());
	let lang = LANGS.get(ext);

	match lang {
		None => None,
		Some(lang) => {
			let mut in_multiline_comment = false;

			// variables to manage the current string
			let mut str_close: Option<String> = None;
			let mut close_escape: Option<String> = None;
			let mut backslash_escape: Option<String> = None;

			let mut is_sloc = false;
			let mut lines = 0;
			while let Some(character) = chars.peek() {
				let character = *character;
				if character == '\n' {
					if is_sloc {
						lines += 1;
						is_sloc = false;
					}

					chars.next();
				} else if in_multiline_comment {
					// check if we have reached the end of the multiline comment
					let end_comment = lang.end_comment.unwrap();
					if peek_string(&mut chars, end_comment) {
						in_multiline_comment = false;
					} else {
						chars.next();
					}
				} else if let Some(ref close) = str_close {
					// str_close is `Some` if and only if we are in a string

					if let Some(ref escape) = backslash_escape {
						// check if the next two characters are escaping a backslash
						// if they are, they should be skipped
						if peek_string(&mut chars, escape) {
							is_sloc = true;
							continue;
						}
					}
					if let Some(ref escape) = close_escape {
						// skip the next two characters if they escaping the end of a string
						// (such as \")

						if peek_string(&mut chars, escape) {
							is_sloc = true;
							continue;
						}
					}

					// check if we have reached the end of the string
					if peek_string(&mut chars, close) {
						str_close = None;
					} else {
						// treat the character as any other character
						is_sloc |= !character.is_whitespace();

						chars.next();
					}
				} else if match lang.single_line_comment {
					Some(comment) => peek_string(&mut chars, comment),
					None => false,
				} {
					// skip the rest of this line
					advance_up_to(&mut chars, '\n');
				} else {
					match lang.string_mode {
						StringMode::Rust => {
							if peek_string(&mut chars, "r#\"") {
								is_sloc = true;
								str_close = Some("\"#".to_owned());

								continue;
							} else if peek_string(&mut chars, r#"r""#) {
								is_sloc = true;
								str_close = Some(r#"""#.to_owned());

								continue;
							} else if peek_string(&mut chars, r#"""#) {
								is_sloc = true;
								str_close = Some(r#"""#.to_owned());
								close_escape = Some(r#"\""#.to_owned());
								backslash_escape = Some(r"\\".to_owned());

								continue;
							}
						}
						StringMode::Normal => {
							if peek_string(&mut chars, r#"""#) {
								is_sloc = true;
								str_close = Some(r#"""#.to_owned());
								close_escape = Some(r#"\""#.to_owned());
								backslash_escape = Some(r"\\".to_owned());

								continue;
							}
						}
					}

					if let Some(comment) = lang.start_comment {
						if peek_string(&mut chars, comment) {
							in_multiline_comment = true;
							continue;
						}
					}

					// if this is not any special character, count this as a
					// line as long as it is not whitespace
					is_sloc |= !character.is_whitespace();

					chars.next();
				}
			}
			// check if the last line is a SLOC
			if is_sloc {
				lines += 1;
			}

			Some(lines)
		}
	}
}
