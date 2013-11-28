use token:: { Token, Add, Sub, Mul, Div, Number, OpenParentheses, CloseParentheses, EOF, Unknown, Identifier, Assign };
use std::char::{ is_digit, is_whitespace, is_alphabetic };
use std::from_str::from_str;
 
// Struct for saving all Tokens and the plain text.
// Plain text can be useful for providing nice error messages.
pub struct Lexer {
	tokens: ~[Token],
	text: ~str
}

impl Lexer {

	// Static method to create a Lexer.
	pub fn new(text: ~str) -> Lexer {

		// Create a lexer and parse the code.
		let mut lexer = Lexer{tokens: ~[], text: text};
		lexer.analyse();

		return lexer;
	}

	fn analyse(&mut self) {

		//  Iterate through every character in the text.
		//  Filter the token out and push it to tokens.
		while self.text.len() >= 1 {

			let ch = self.text.shift_char();
			
			// Ignore white spaces.
			if !is_whitespace(ch) {

				// Filter the token out of the text and push it into the tokens.
				let token = self.filter_token_out(ch); 
				self.tokens.push(token);

			}
		}

		// Push EOF to tokens what indicates the end.
		self.tokens.push(Token(EOF)) ;

	}

	fn filter_token_out(&mut self, ch: char) -> Token {

		match ch {
			'+' => Token(Add),
			'-' => Token(Sub),
			'*' => Token(Mul),
			'/' => Token(Div),
			'(' => Token(OpenParentheses),
			')' => Token(CloseParentheses),
			'=' => Token(Assign),
			d if is_digit(d) => { // if ch e.g. is '3' then the number could be "3.5".
				self.filter_number_out(d)
			}
			a if is_alphabetic(a)   => { // if ch is 'a' or 'x' it indicates a identifier.
				self.filter_identifier_out(a)	
			}
			_   => { // ch don't indicate a token.
				error!("[error: lexer.rs in Lexer::filter_token_out] -> {} is a unknown character.", ch);
				Token(Unknown(ch))
			}
		} 

	}

	fn filter_identifier_out(&mut self, ch: char) -> Token {
		
		let mut identifier = ~""; // Temporary string where the identifier is stored.

		// Push the first given character to our temporary identifier string.
		identifier.push_char(ch);

		// Iterate through the text until we hit the end of the identifier.
		// So we pushed every character of the identifier into the identifier string.
		loop {
			if self.text.len() < 1 { // Prevents errors like index is out of range.
				break
			}

			// Get the next character.
			let ch = self.text.char_at(0);

			if is_alphabetic(ch) { // An identifier only contains alphabetic characters.
				identifier.push_char(ch)
			} else { // We hit the end of the identifier.
				break
			}

			self.text.shift_char();

		}

		return Token(Identifier(identifier))
	}

	fn filter_number_out(&mut self, ch: char) -> Token {

		let mut number = ~""; // Temporary string where the number string is stored.

		// Push the first given character to our temporary number string.
		number.push_char(ch);

		// Iterate through the text until we hit the end of the number.
		// So we pushed every character of the number into the number string.
		loop {
			if self.text.len() < 1 { // Prevents errors like index is out of range.
				break
			}

			// Get the next character.
			let ch = self.text.char_at(0);

			if is_digit(ch) || ch == '.' { // A number can only contain digits and a decimal point.
				number.push_char(ch)
			} else { // We hit the end of the number.
				break
			}

			self.text.shift_char();

		}

		// Convert the number string into a real number.
		let n = match from_str::<f64>(number) {
			Some(n) => n,
			None    => {
				error!("[error: lexer.rs in Lexer::filter_number_out] -> couldn't convert {} into a floating point number!", number);
				return Token(Unknown('X'))
			}

		};

		return Token(Number(n));
	}
}
