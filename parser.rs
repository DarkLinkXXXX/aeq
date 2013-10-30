pub mod parser {

    pub struct Token {
        id: Tokens
    }

    impl Token {
        pub fn print(&self) {
            match self.id {
                Constant(Number(n))     => println!("{}", n),
                Operator(Add)           => println("Add"),
                Operator(Sub)           => println("Sub"),
                Operator(Mul)           => println("Mul"),
                Operator(Div)           => println("Div"),
                Parenthese(Open)        => println("Parenthese Open"),
                Parenthese(Close)       => println("Parenthese Close"),
                Unknown(c)              => println!("Unknown({})", c)
            }
        }
    }

    pub enum Tokens {
        Constant(Constants),
        Operator(Operators),
        Parenthese(Parentheses),
        Unknown(char)
    }

    pub enum Constants {
        Number(f64)
    }

    pub enum Operators {
        Add, Sub, Mul, Div
    }

    pub enum Parentheses {
        Open, Close
    }

    pub mod tokenizer {

        use super::Token;

        pub fn each(t: &[Token], op: &fn(t: &Token)) {
            let mut n = 0u;
            while n < t.len() {
                op(&t[n]);
                n += 1;
            }
        }

        pub fn iter(s: ~str, op: &fn(c: char, next: &mut uint, s: &str)) {
            use std::str::*;
            let mut i = 0u;
            while i < s.len() {
                let ch = s.char_range_at(i).ch;
                let mut next = s.char_range_at(i).next;
                op(ch, &mut next, s);
                i = next
            }
        }

    }

    pub fn tokenizer(expr: ~str) -> ~[Token] {
        use std::char::is_digit;
        use std::from_str::from_str;

        let mut tokens: ~[Token] = ~[];
        
        do tokenizer::iter(expr) |ch, next, expr| {
            let token = match ch {
                '+' => Token{ id: Operator(Add) },
                '-' => Token{ id: Operator(Sub) },
                '*' => Token{ id: Operator(Mul) },
                '/' => Token{ id: Operator(Div) },
                '(' => Token{ id: Parenthese(Open) },
                ')' => Token{ id: Parenthese(Close) },
                d if is_digit(d) => {
                    let mut substr = ~"";
                    substr.push_char(d);
                    loop {
                        if *next >= expr.len() {
                            break
                        }
                        let ch = expr.char_range_at(*next).ch;
                        if is_digit(ch) || ch == '.' {
                            substr.push_char(ch)
                        } else {
                            break
                        }
                        *next = expr.char_range_at(*next).next;
                    }
                    let n = match from_str::<f64>(substr) {
                        Some(val) => val,
                        None => -1f64
                    };
                    Token{ id: Constant(Number(n)) }
                }
                _ => Token{ id: Unknown(ch) }
            }; 
            tokens.push(token);
        }
        tokens
    }
}
