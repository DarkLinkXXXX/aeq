use token::{ Token, EOF };
use lexer::Lexer;
use std::fmt;

pub struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>
}

pub struct Parser {
	root: ~Node,
	tokens: ~[Token]
}

impl fmt::Default for Node {
	fn fmt(obj: &Node, f: &mut fmt::Formatter) {
		let left = match obj.left {
			Some(ref x) => format!("{}", **x),
			None => ~"None"
		};
		let right = match obj.right {
			Some(ref x) => format!("{}", **x),
			None => ~"None"
		};
		write!(f.buf, "Node : T({}), L[{}], R({})", obj.token, left, right)
	}
}

impl Parser {
	pub fn new(lexer: Lexer) -> Parser {
		let tmp_tokens = lexer.tokens.clone();
		let mut parser = Parser{ root: ~Node{ token: Token(EOF), left: None, right: None }, tokens: lexer.tokens };
		parser.root = ~parser.parse_expression(Node{token: tmp_tokens[0], left: None, right: None}, 0);
		return parser;
	}

	fn parse_expression(&mut self, mut lhs: Node, min_precedence: uint) -> Node {

		// lhs: left-hand-side, rhs: right-hand-side

		if self.tokens[0] == lhs.token { // lhs is not lookahead(lh)
			self.tokens.shift();
		}
		
		let mut lh = self.tokens[0]; // look at the next token, our lookahead

		while lh.is_operator() && lh.precedence() >= min_precedence {
			
			let op = self.tokens.shift(); // get next token, our operator
			let mut rhs = Node{ token: self.tokens.shift(), left: None, right: None }; // get next token, our rhs
			lh = self.tokens[0];

			while lh.is_operator() && lh.precedence() > op.precedence() {
				
				// recursive invocation of parse_expression 
				rhs = self.parse_expression(rhs, lh.precedence());
				lh = self.tokens[0];

			}

			// save our results in the lhs
			lhs = Node{ token: op, left: Some(~lhs), right: Some(~rhs) };
			lh = self.tokens[0];

		}

		return lhs;

	}
}
