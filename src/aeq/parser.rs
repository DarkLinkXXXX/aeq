use token::{ Token, EOF, OpenParentheses, Number, Identifier };
use lexer::Lexer;
use std::fmt;

// Struct to build a parse tree.
#[deriving(Eq, Clone)]
pub struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>
}

// Struct for the parser.
pub struct Parser {
	root: ~Node,
	tokens: ~[Token]
}

// Implements format!("{}", Node).
// What makes it easy to debug code.
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

		// Save tokens temporary, because we move them later.
		let tmp_tokens = lexer.tokens.clone();

		// Create a parser and parse the first lhs of the expression. Tokens are moved.
		let mut parser = Parser{ root: ~Node{ token: Token(EOF), left: None, right: None }, tokens: lexer.tokens };
		let lhs = parser.parse_operand(Node {token: tmp_tokens[0], left: None, right: None}, 0);

		// Parse our expression and save the root node
		parser.root = ~parser.parse_expression(lhs, 0);

		return parser;

	}

	// Method for parsing expressions inside brackets.
	fn parse_operand(&mut self, mut lhs: Node, min_precedence: uint) -> Node {
		
		match *lhs.token {
			
			OpenParentheses => {

				self.tokens.shift();	// eat '('.
				lhs.token =  self.tokens[0].clone();	// (a+b) lhs.tokens = a.

				// parse the hole expression.
				let node = self.parse_expression(lhs, min_precedence);

				self.tokens.shift(); // eat ')'.
				return node 
			}

			Number(_)	=> { 
				self.tokens.shift(); // eat lhs.
				return lhs 
			}
			
			Identifier(_)   => {
				self.tokens.shift(); // eat lhs.
				return lhs;
			}

			_		=> { 
				error!("[error: parser.rs in Parser::parse_operand] -> Unexpected token: {}", lhs.token);
				self.tokens.shift(); 
				return lhs 
			}
		}

	}

	fn parse_expression(&mut self, mut lhs: Node, min_precedence: uint) -> Node {

		// lhs: left-hand-side, rhs: right-hand-side:

		if self.tokens[0] == lhs.token { // lhs is not lookahead(lh).
			self.tokens.shift();
		}
		
		let mut lh = self.tokens[0].clone(); // look at the next token, our lookahead.

		while lh.is_operator() && lh.precedence() >= min_precedence {
			
			let op = self.tokens.shift(); // get next token, our operator.

			let mut rhs = Node{ token: self.tokens[0].clone(), left: None, right: None }; // get next token, our rhs.
			rhs = self.parse_operand(rhs, 0); // if rhs is a expression in brackets then parse it e.g. 3+(3+3).

			lh = self.tokens[0].clone();

			while lh.is_operator() && lh.precedence() > op.precedence() {
				
				// recursive invocation of parse_expression.
				rhs = self.parse_expression(rhs, lh.precedence());
				lh = self.tokens[0].clone();

			}

			// save our results in the lhs
			lhs = Node{ token: op, left: Some(~lhs), right: Some(~rhs) };
			lh = self.tokens[0].clone();

		}

		return lhs;

	}
}
