use token:: { Token, Add, Sub, Mul, Div, Number, OpenParentheses, CloseParentheses, EOF, Unknown, iter };
use std::char::is_digit;
use std::from_str::from_str;

pub struct Lexer {
	tokens: ~[Token],
	text: ~str
}

impl Lexer {

	pub fn new(text: ~str) -> Lexer {
		let mut lexer = Lexer{tokens: ~[], text: text};
		lexer.analyse();
		return lexer;
	}

	fn analyse(& mut self) {

		//  Iterate through every character in the text.
		//  Check if the character is a token or could indicate some token.
		do iter(self.text) |ch, next| {

			let token = match ch {
				'+' => { Token(Add) }
				'-' => { Token(Sub) }
				'*' => { Token(Mul) }
				'/' => { Token(Div) }
				'(' => { Token(OpenParentheses) }
				')' => { Token(CloseParentheses) }

				d if is_digit(d) => {
					match token_number(d, next, self.text) {
						Some(t) => { t }
						None    => {
							warn!("warning: token.rs in tokenizer: couldn't match token_number!");
							Token(Unknown(ch))
						}
					}
				}

				_   => {
					info!("info: token.rs in tokenizer: {} is a unknown character.", ch);
					Token(Unknown(ch))
				}
			}; 

			self.tokens.push(token);
		}

		self.tokens.push(Token(EOF));

	}

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
			warn!("warning: token.rs in token_number: couldn't convert {} into a floating point number!", number);
			return None
		}
	};

	return Some(Token(Number(n)));
}
