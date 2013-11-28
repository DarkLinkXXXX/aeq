use std::fmt;

// We define our data structure for our token with a simple tuple struct.
#[deriving(Eq, Clone)]
pub struct Token(Tokens);

// Enumeration for all existing tokens.
#[deriving(Eq, Clone)]
pub enum Tokens {
	Add, Sub, Mul, Div,
	Assign,
	Number(f64),
	OpenParentheses,
	CloseParentheses,
	Identifier(~str),
	EOF,
	Unknown(char)
}

// Implements format!("{}", token) for easy printing and debugging.
impl fmt::Default for Token {

	fn fmt(obj: &Token, f: &mut fmt::Formatter) {

		let txt = match **obj {
			Add => ~"+", Sub => ~"-", Mul => ~"*", Div => ~"/",
			Assign => ~"=",
			Number(x) => format!("{}", x),
			OpenParentheses => ~"(",
			CloseParentheses => ~")",
			Identifier(ref x) => x.clone(),
			EOF => ~"EOF",
			Unknown(x) => format!("{}", x)
		};

		write!(f.buf, "{}", txt)

	}
}

impl Token {

	// Method that returns the precedence of a operator.
	pub fn precedence(&self) -> uint {

		match **self {
			Add => 2, Sub => 2,
			Mul => 3, Div => 3,
			Assign => 1,
			_ => 0
		}

	}

	// Method that tells us if the token is an operator.
	pub fn is_operator(&self) -> bool {

		match **self {
			Add => true, Sub => true,
			Mul => true, Div => true,
			Assign => true,
			_ => false
		}

	}

}

// Method used by the Lexer in lexer.rs for iterating through the text.
pub fn iter(text: &str, op: proc(c: char, next: &mut uint)) {

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
