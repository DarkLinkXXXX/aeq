use token:: { Token, Add, Sub, Mul, Div, Number, OpenParentheses, CloseParentheses, EOF, Unknown, iter };
use std::char::is_digit;
use std::from_str::from_str;
 
// Struct for saving all Tokens and the plain text.
// Plain text can be useful for providing nice error messages.
pub struct Lexer {
	tokens: ~[Token],
	text: ~str
}

impl Lexer {

	// Method to create a Lexer
	pub fn new(text: ~str) -> Lexer {
		let mut lexer = Lexer{tokens: ~[], text: text};
		lexer.analyse();
		return lexer;
	}

	fn analyse(& mut self) {

		//  Iterate through every character in the text.
		//  Check if the character is a token or could indicate some token.
		do iter(self.text) |ch, next| {

			// Filter the token out of the text and push it into the tokens 
			let token = self.filter_token_out(ch, next); 

			self.tokens.push(token);
		}

		// Push EOF to tokens what indicates the end
		self.tokens.push(Token(EOF)) ;

	}

	fn filter_token_out(&self, ch: char, next: &mut uint) -> Token {
		match ch {
			'+' => Token(Add),
			'-' => Token(Sub),
			'*' => Token(Mul),
			'/' => Token(Div),
			'(' => Token(OpenParentheses),
			')' => Token(CloseParentheses),
			d if is_digit(d) => { // if ch eg. is '3' and the number is "3.5"
				self.filter_number_out(d, next)
			}
			_   => { // ch don't indicate a token	
				warn!("info: token.rs in tokenizer: {} is a unknown character.", ch);
				Token(Unknown(ch))
			}
		} 
	}

	fn filter_number_out(&self, ch: char, next: &mut uint) -> Token {

		let mut number = ~"";

		// push the first given character ch e.g. 7.88 -> '7' would be ch
		// into our number string
		number.push_char(ch);

		// Iterate through the text until we hit the end of the number.
		// So we pushed every character of the number into the number string.
		loop {
			if *next >= self.text.len() { // prevends errors like next is out of range
				break
			}

			let ch = self.text.char_range_at(*next).ch;

			if is_digit(ch) || ch == '.' {
				number.push_char(ch)
			} else {
				break
			}

			*next = self.text.char_range_at(*next).next;
		}

		// convert the number string into a real number
		let n = match from_str::<f64>(number) {
			Some(n) => n,
			None    => {
				warn!("warning: token.rs in token_number: couldn't convert {} into a floating point number!", number);
				return Token(Unknown(ch))
			}
		};

		return Token(Number(n));
	}
}

#[test]
fn test_lexer() {
	let lexer = Lexer::new(~"3+3*7");
	if lexer.tokens != ~[Token(Number(3f64)), Token(Add), Token(Number(3f64)), Token(Mul), Token(Number(7f64)), Token(EOF)] {
		fail!("test \"{}\": lexer::new failed!", lexer.text)
	}
} 
