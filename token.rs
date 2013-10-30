pub mod token {

    pub struct Token {
        id: Tokens
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

        use super::*;

        pub fn get_tokens(text: ~str) -> ~[Token] {
            use std::char::is_digit;

            let mut tokens: ~[Token] = ~[];
            
            do iter(text) |ch, next| {
                let token = match ch {
                    '+' => Token{ id: Operator(Add) },
                    '-' => Token{ id: Operator(Sub) },
                    '*' => Token{ id: Operator(Mul) },
                    '/' => Token{ id: Operator(Div) },
                    '(' => Token{ id: Parenthese(Open) },
                    ')' => Token{ id: Parenthese(Close) },
                    d if is_digit(d) => {
                        match token_number(d, next, text) {
                            Some(t) => t,
                            None => Token{ id: Unknown(ch) }
                        }
                    }
                    _ => Token{ id: Unknown(ch) }
                }; 
                tokens.push(token);
            }

            tokens
        }

        fn token_number(ch: char, next: &mut uint, text: &str) -> Option<Token> {
            use std::char::is_digit;
            use std::from_str::from_str;

            let mut substr = ~"";
            substr.push_char(ch);
            loop {
                if *next >= text.len() {
                    break
                }
                let ch = text.char_range_at(*next).ch;
                if is_digit(ch) || ch == '.' {
                    substr.push_char(ch)
                } else {
                    break
                }
                *next =  text.char_range_at(*next).next;
            }
            let n = match from_str::<f64>(substr) {
                Some(val) => val,
                None => return None
            };

            Some(Token{ id: Constant(Number(n)) })
        }

        fn iter(text: &str, op: &fn(c: char, next: &mut uint)) {
            let mut i = 0u;
            while i < text.len() {
                let ch = text.char_range_at(i).ch;
                let mut next = text.char_range_at(i).next;
                op(ch, &mut next);
                i = next
            }
        }

        pub fn each(t: &[Token], op: &fn(t: &Token)) {
            let mut n = 0u;
            while n < t.len() {
                op(&t[n]);
                n += 1;
            }
        }
    } 
}
