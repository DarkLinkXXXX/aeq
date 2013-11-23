use token:: { Add, Sub, Mul, Div, Number };
use parser;

// Implement a interpret method to interpret our parse tree.
impl parser::Node {

	pub fn interpret(&self) -> f64{

		let left = match self.left {
			Some(ref n) => n.interpret(), // recursive invocation.
			None        => 1f64
		};

		let right = match self.right {
			Some(ref n) => n.interpret(), // recursive invocation.
			None        => 1f64
		};

		match *self.token {

			Add => left + right,
			Sub => left - right,
			Mul => left * right,
			Div => left / right,
			Number(n) => n,	
			_   => { error!("[error: interpreter.rs in Node::interpret] -> Unknown Token to interpret."); return 0f64 }

		}

	}

}	
