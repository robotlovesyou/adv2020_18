use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

#[derive(Copy, Clone, Debug)]
enum Token {
    Int(i64),
    Add,
    Mul,
    LParen,
    RParen,
}

struct Tokenizer {
    chars: Peekable<IntoIter<char>>,
}

impl Tokenizer {
    pub fn new(source: Vec<char>) -> Tokenizer {
        Tokenizer {
            chars: source.into_iter().peekable(),
        }
    }

    fn parse_num(&mut self, first: char) -> Token {
        let mut buf = String::from(first);
        while let Some(n) = self.chars.peek() {
            match n {
                c @ '0'..='9' => {
                    buf.push(*c);
                    self.chars.next();
                }
                _ => break,
            }
        }
        Token::Int(buf.parse::<i64>().unwrap())
    }
}

trait IntoTokens {
    fn into_tokens(self) -> Tokenizer;
}

impl IntoTokens for Chars<'_> {
    fn into_tokens(self) -> Tokenizer {
        Tokenizer::new(self.collect())
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.chars.next() {
            use self::Token::*;
            let next = match c {
                '+' => Some(Add),
                '*' => Some(Mul),
                '(' => Some(LParen),
                ')' => Some(RParen),
                first @ '0'..='9' => Some(self.parse_num(first)),
                _ => None,
            };
            if next.is_some() {
                return next;
            }
        }
        None
    }
}

use self::Token::*;

fn eval(expression: &str) -> i64 {
    _eval(&mut expression.chars().into_tokens().peekable())
}

fn _eval(tokens: &mut Peekable<Tokenizer>) -> i64 {
    let mut current = eval_operand(tokens);
    while tokens.peek().is_some() {
        current = eval_operation(current, tokens);
    }
    current
}

fn eval_operand(tokens: &mut Peekable<Tokenizer>) -> i64 {
    match tokens
        .next()
        .expect("unexpected end of tokens evaluating operand")
    {
        Int(n) => n,
        LParen => eval_subexpr(tokens),
        other => panic!("illegal token in operand {:?}", other),
    }
}

fn eval_subexpr(tokens: &mut Peekable<Tokenizer>) -> i64 {
    let mut current = eval_operand(tokens);
    while let Some(peeked) = tokens.peek() {
        match peeked {
            RParen => {
                tokens.next();
                return current;
            }
            _ => current = eval_operation(current, tokens),
        }
    }
    current
}

fn eval_operation(loper: i64, tokens: &mut Peekable<Tokenizer>) -> i64 {
    let operation = tokens
        .next()
        .expect("unexpected end of tokens evaluating operation");
    let roper = eval_operand(tokens);

    match operation {
        Add => loper + roper,
        Mul => loper * roper,
        other => panic!("{:?} is not an operation", other),
    }
}

//fval because f comes after e...
fn fval(line: &str) -> i64 {
    _fval(&mut line.chars().into_tokens().peekable())
}

fn _fval(tokens: &mut Peekable<Tokenizer>) -> i64 {
    let mut current = fval_loperand(tokens);
    while tokens.peek().is_some() {
        current = fval_operation(current, tokens);
    }
    current
}

fn fval_subexpr(tokens: &mut Peekable<Tokenizer>) -> i64 {
    let mut current = fval_loperand(tokens);
    while let Some(peeked) = tokens.peek() {
        match peeked {
            RParen => {
                tokens.next();
                return current;
            }
            _ => current = fval_operation(current, tokens),
        }
    }
    current
}

fn fval_loperand(tokens: &mut Peekable<Tokenizer>) -> i64 {
    match tokens
        .next()
        .expect("unexpected end of tokens evaluating operand")
    {
        Int(n) => n,
        LParen => fval_subexpr(tokens),
        other => panic!("illegal token in operand {:?}", other),
    }
}

fn fval_roperand(tokens: &mut Peekable<Tokenizer>) -> i64 {
    let roper = match tokens
        .next()
        .expect("unexpected end of tokens evaluating operand")
    {
        Int(n) => n,
        LParen => fval_subexpr(tokens),
        other => panic!("illegal token in operand {:?}", other),
    };
    if matches!(tokens.peek(), Some(Add)) {
        tokens.next();
        fval_addition(roper, tokens)
    } else {
        roper
    }
}

fn fval_addition(loper: i64, tokens: &mut Peekable<Tokenizer>) -> i64 {
    loper + fval_roperand(tokens)
}

fn fval_operation(loper: i64, tokens: &mut Peekable<Tokenizer>) -> i64 {
    let operation = tokens
        .next()
        .expect("unexpected end of tokens evaluating operation");
    let roper = fval_roperand(tokens);

    match operation {
        Add => loper + roper,
        Mul => loper * roper,
        other => panic!("{:?} is not an operation", other),
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let part_1: i64 = input.lines().map(eval).sum();
    println!("The answer to part 1 is {}", part_1);

    let part_2: i64 = input.lines().map(fval).sum();
    println!("The answer to part 2 is {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_produces_correct_tokens() {
        let mut tokenizer = "1()+*".chars().into_tokens();
        assert!(matches!(tokenizer.next(), Some(Token::Int(1))));
        assert!(matches!(tokenizer.next(), Some(Token::LParen)));
        assert!(matches!(tokenizer.next(), Some(Token::RParen)));
        assert!(matches!(tokenizer.next(), Some(Token::Add)));
        assert!(matches!(tokenizer.next(), Some(Token::Mul)));
        assert!(matches!(tokenizer.next(), None));
    }

    #[test]
    fn it_can_eval_a_literal_expression() {
        let result = eval("123");
        assert_eq!(123, result);
    }

    #[test]
    fn it_can_eval_an_addition() {
        let result = eval("10 + 11");
        assert_eq!(21, result);
    }

    #[test]
    fn it_can_eval_a_multiplication() {
        assert_eq!(110, eval("10 * 11"));
    }

    #[test]
    fn it_can_eval_a_chained_expression() {
        assert_eq!(105, eval("10 + 11 * 5"))
    }

    #[test]
    fn it_can_eval_simple_subexpressions() {
        let result = eval("(10) + (11)");
        assert_eq!(21, result);
    }

    #[test]
    fn it_can_eval_subexpressions_with_arithmatic() {
        let result = eval("(10 + 10) + (11 * 11)");
        assert_eq!(141, result);
    }

    #[test]
    fn examples_from_question_parse_correctly() {
        assert_eq!(26, eval("2 * 3 + (4 * 5)"));
        assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            13632,
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn it_can_advanced_eval_literal_expressions() {
        assert_eq!(123, fval("123"));
    }

    #[test]
    fn it_can_advanced_eval_addition() {
        assert_eq!(21, fval("10 + 11"));
    }

    #[test]
    fn it_can_advanced_eval_multiplication() {
        assert_eq!(110, fval("10 * 11"));
    }

    #[test]
    fn it_can_advanced_eval_chained_operations() {
        assert_eq!(230, fval("10 * 11 + 12"));
    }

    #[test]
    fn it_can_advanced_eval_examples_from_the_question() {
        assert_eq!(231, fval("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, fval("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, fval("2 * 3 + (4 * 5)"));
        assert_eq!(1445, fval("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669060, fval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            23340,
            fval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
