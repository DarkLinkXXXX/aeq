use token::{ Token, EOF, OpenParentheses, CloseParentheses, Number };
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
		let lhs = parser.parse_operand(Node {token: tmp_tokens[0], left: None, right: None}, 0);
		debug!("{}|{}", lhs, parser.tokens[0]);
		parser.root = ~parser.parse_expression(lhs, 0);
		return parser;
	}

	fn parse_operand(&mut self, mut lhs: Node, min_precedence: uint) -> Node {
		
		match *lhs.token {
			
			OpenParentheses => {
				self.tokens.shift();
				lhs.token =  self.tokens[0];
				let node = self.parse_expression(lhs, min_precedence);
				debug!("operand: {}", node);
				self.tokens.shift();
				node
			} 
			Number(n)	=> { self.tokens.shift(); lhs }
			_		=> { error!("Unexpected token: {}", lhs.token); self.tokens.shift(); lhs }
		}

	}

	fn parse_expression(&mut self, mut lhs: Node, min_precedence: uint) -> Node {

		// lhs: left-hand-side, rhs: right-hand-side

		if self.tokens[0] == lhs.token { // lhs is not lookahead(lh)
			self.tokens.shift();
		}
		
		let mut lh = self.tokens[0]; // look at the next token, our lookahead

		while lh.is_operator() && lh.precedence() >= min_precedence {

			debug!("lhs: {}", lhs);
			
			let op = self.tokens.shift(); // get next token, our operator
			debug!("op: {}", op);
			let mut rhs = Node{ token: self.tokens[0], left: None, right: None }; // get next token, our rhs
			rhs = self.parse_operand(rhs, 0);
			debug!("rhs: {}", rhs);
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
