use std::str::Chars;

use itertools::MultiPeek;

/// Advances the iterator up to (but not including) the provided character.
/// Returns `false` if the character could not be found by the end of the string.
pub fn advance_up_to(iter: &mut MultiPeek<Chars>, target: char) -> bool {
	iter.reset_peek();
	while let Some(character) = iter.peek() {
		if *character == target {
			return true;
		} else {
			iter.next();
		}
	}

	false
}

/// Checks if the next characters of the iterator match the string.
/// If it does, this function will move the iterator past the end of the matching substring.
pub fn peek_string(iter: &mut MultiPeek<Chars>, target: &str) -> bool {
	if let Some(string) = try_peek_chars(iter, target.len()) {
		if string == *target {
			for _ in 0..string.len() {
				iter.next();
			}

			return true;
		}
	}

	false
}

pub fn try_peek_chars(iter: &mut MultiPeek<Chars>, amt: usize) -> Option<String> {
	let mut string = String::with_capacity(amt);
	iter.reset_peek();

	while string.len() < amt {
		match iter.peek() {
			Some(character) => string.push(*character),
			None => return None,
		};
	}

	Some(string)
}
