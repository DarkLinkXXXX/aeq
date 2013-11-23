use std::fmt;

#[deriving(Eq)]
pub struct Token(Tokens);

#[deriving(Eq)]
pub enum Tokens {
	Add, Sub, Mul, Div,
	Number(f64),
	OpenParentheses,
	CloseParentheses,
	EOF,
	Unknown(char)
}

impl fmt::Default for Token {
	fn fmt(obj: &Token, f: &mut fmt::Formatter) {
		let txt = match **obj {
			Add => ~"+", Sub => ~"-", Mul => ~"*", Div => ~"/",
			Number(x) => format!("{}", x),
			OpenParentheses => ~"(",
			CloseParentheses => ~")",
			EOF => ~"EOF",
			Unknown(x) => format!("{}", x)
		};

		write!(f.buf, "{}", txt)
	}
}

impl Token {

	pub fn precedence(&self) -> uint {
		match **self {
			Add => 1, Sub => 1,
			Mul => 2, Div => 2,
			_ => 0
		}
	}

	pub fn is_operator(&self) -> bool {
		match **self {
			Add => true, Sub => true,
			Mul => true, Div => true,
			_ => false
		}
	}
}



pub fn iter(text: &str, op: &fn(c: char, next: &mut uint)) {

	let mut n = 0u;

	// Iterate through every character of the text an issue
	// the given closure on it.
	while n < text.len() {
		let ch = text.char_range_at(n).ch;
		let mut next = text.char_range_at(n).next;

		op(ch, &mut next);
		n = next
	}
}
