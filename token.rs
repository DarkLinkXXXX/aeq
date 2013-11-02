use std::char::is_digit;
use std::from_str::from_str;

#[deriving(Eq)]
pub struct Token {
	id: Tokens
}

#[deriving(Eq)]
pub enum Tokens {
	Constant(Constants),
	Operator(Operators),
	Parenthesis(Parentheses),
	Unknown(char)
}

#[deriving(Eq)]
pub enum Constants {
	Number(f64)
}

#[deriving(Eq)]
pub enum Operators {
	Add, Sub, Mul, Div
}

#[deriving(Eq)]
pub enum Parentheses {
	Open, Close
}

pub fn tokenizer(text: &str) -> ~[Token] {

	let mut tokens: ~[Token] = ~[];

	//  Iterate through every character in the text.
	//  Check if the character is a token or could indicate some token.
	do iter(text) |ch, next| {

		let token = match ch {
			'+' => { Token{ id: Operator(Add) } }
			'-' => { Token{ id: Operator(Sub) } }
			'*' => { Token{ id: Operator(Mul) } }
			'/' => { Token{ id: Operator(Div) } }
			'(' => { Token{ id: Parenthesis(Open) } }
			')' => { Token{ id: Parenthesis(Close) } }

			d if is_digit(d) => {
				match token_number(d, next, text) {
					Some(t) => { t }
					None    => {
						warn!("warning: token.rs in tokenizer: couldn't match token_number!");
						Token{ id: Unknown(ch) } 
					}
				}
			}

			_   => {
				info!(format!("info: token.rs in tokenizer: {} is a unknown character.", ch));
				Token{ id: Unknown(ch) } 
			}
		}; 

		tokens.push(token);
	}

	return tokens;
}

fn token_number(ch: char, next: &mut uint, text: &str) -> Option<Token> {

	let mut number = ~"";

	// push the first given character ch e.g. 7.88 -> '7' would be ch
	// into our number string
	number.push_char(ch);

	// Iterate through the text until we hit the end of the number.
	// So we pushed every character of the number into the number string.
	loop {
		if *next >= text.len() {
			break
		}

		let ch = text.char_range_at(*next).ch;

		if is_digit(ch) || ch == '.' {
			number.push_char(ch)
		} else {
			break
		}

		*next = text.char_range_at(*next).next;
	}

	// convert the number string into a real number
	let n = match from_str::<f64>(number) {
		Some(n) => n,
		None    => {
			warn!(format!("warning: token.rs in token_number: couldn't convert {} into a floating point number!", number));
			return None
		}
	};

	return Some(Token{ id: Constant(Number(n)) });
}

fn iter(text: &str, op: &fn(c: char, next: &mut uint)) {

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

pub fn each(t: &[Token], op: &fn(t: &Token)) {

	let mut n = 0u;

	// Iterate through every token and issue the given closure on it.
	while n < t.len() {
		op(&t[n]);
		n += 1;
	}
}

// Unit tests to check if our function do want we want them to do

#[test]
fn test_tokenizer() {

	// Check for diffrent expressions
	let expr = "(3.3/5.5)";
	let tokens = ~[ Token{ id: Parenthesis(Open) }, Token{ id: Constant(Number(3.3f64)) }, Token{ id: Operator(Div) }, Token{ id: Constant(Number(5.5f64)) }, Token{ id: Parenthesis(Close) }]; 
	if tokenizer(expr) != tokens {
		fail!(format!("test: token.rs in test_tokenizer: couldn't tokenize \"{}\"", expr)) 
	}

	let expr = "(3+3)*3";
	let tokens = ~[ Token{ id: Parenthesis(Open) }, Token{ id: Constant(Number(3f64)) }, Token{ id: Operator(Add) }, Token{ id: Constant(Number(3f64)) }, Token{ id: Parenthesis(Close) }
	, Token{ id: Operator(Mul) }, Token{ id: Constant(Number(3f64)) }];
	if tokenizer(expr) != tokens {
		fail!(format!("test: token.rs in test_tokenizer: couldn't tokenize \"{}\"", expr)) 
	}
}

#[test]
fn test_token_number() {
	let number = "7.88";
	match token_number(number.char_range_at(0).ch, &mut number.char_range_at(0).next, number) {
		Some(t) => { if t == Token{ id: Constant(Number(7.88f64)) } { return } }
		None    => ()
	}

	let number = "7";
	match token_number(number.char_range_at(0).ch, &mut number.char_range_at(0).next, number) {
		Some(t) => { if t == Token{ id: Constant(Number(7f64)) } { return } }
		None    => ()
	}
	fail!(format!("test: token.rs in test_token_number: couldn't token_number \"{}\"", number))
}
