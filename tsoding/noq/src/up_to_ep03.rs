use std::{fmt, collections::HashMap, io::{self, stdin, stdout, Write}, iter::Peekable};
use Expr::*;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Sym(String),
    Fun(String, Vec<Expr>)
}

impl Expr {
    fn parse_peekable(lexer: &mut Peekable<impl Iterator<Item=Token>>) -> Self {
        if let Some(name) =  lexer.next() {
            match name.kind {
                TokenKind::Sym => {
                    if let Some(_) = lexer.next_if(|t| t.kind == TokenKind::OpenParen) {
                        let mut args = Vec::new();
                        if let Some(_) = lexer.next_if(|t| t.kind == TokenKind::CloseParen) {
                            return Fun(name.text, args)
                        }
                        args.push(Self::parse_peekable(lexer));
                        while let Some(_) = lexer.next_if(|t| t.kind == TokenKind::Comma) {
                            args.push(Self::parse_peekable(lexer));
                        }
                        if lexer.next_if(|t| t.kind == TokenKind::CloseParen).is_none() {
                            todo!("Expected close paren");
                        }
                        return Fun(name.text, args);
                    } else {
                        return Sym(name.text);
                    }
                },
                _ => todo!("Report excted symbol"),
            }
        } else {
            todo!("Report EOF error")
        }
    }
    fn parse(lexer: impl Iterator<Item=Token>) -> Self {
        Self::parse_peekable(&mut lexer.peekable())
    }
}

#[derive(Debug)]
struct Rule {
    head: Expr,
    body: Expr
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Sym(name) => write!(f, "{}", name),
            Expr::Fun(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")? }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

fn substitue_bindings(bindings: &Bindings, expr: &Expr) -> Expr {
    match expr {
        Sym(name) => {
            if let Some(value) = bindings.get(name) {
                value.clone()
            } else {
                expr.clone()
            }
        },
        Fun(name, args) => {
            let new_name = match bindings.get(name) {
                Some(Sym(new_name)) => new_name.clone(),
                None => name.clone(),
                Some(_) => panic!("Expected symbol in plavce of the functor name")
            };
            let mut new_args = Vec::new();
            for arg in args {
                new_args.push(substitue_bindings(bindings, &arg))
            }
            Fun(new_name, new_args)
        }
    }
}

impl Rule {
    fn apply_all(&self, expr: &Expr) -> Expr {
        if let Some(bindings) = pattern_match(&self.head, expr) {
            substitue_bindings(&bindings, &self.body)
        } else {
            // use Expr::*;
            match expr {
                Sym(_) => expr.clone(),
                Fun(name, args) => {
                    let mut new_args = Vec::new();
                    for arg in args {
                        new_args.push(self.apply_all(arg))
                    }
                    Fun(name.clone(), new_args)
                }
            }
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.head, self.body)
    }
}

type Bindings = HashMap<String, Expr>;

fn pattern_match(pattern: &Expr, value: &Expr) -> Option<Bindings> {
    fn pattern_match_impl(pattern: &Expr, value: &Expr, bindings: &mut Bindings) -> bool {
        // use Expr::*;
        match (pattern, value) {
            (Sym(name), _) => {
                if let Some(bound_value) = bindings.get(name) {
                    bound_value == value
                } else {
                    bindings.insert(name.clone(), value.clone());
                    true
                }
            },
            (Fun(name1, args1), Fun(name2, args2)) => {
                if name1 == name2 && args1.len() == args2.len() {
                    for i in 0..args1.len() {
                        if !pattern_match_impl(&args1[i], &args2[i], bindings) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }
    
    let mut bindings = HashMap::new();

    if pattern_match_impl(pattern, value, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Sym,
    OpenParen,
    CloseParen,
    Comma,
    Equals,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    text: String,
}

struct Lexer<Chars: Iterator<Item=char>> {
    chars: Peekable<Chars>,
}

impl<Chars: Iterator<Item=char>> Lexer<Chars> {
    fn from_iter(chars: Chars) -> Self {
        Self { 
          chars: chars.peekable(),
        }
    }
}

impl<Chars: Iterator<Item=char>> Iterator for Lexer<Chars> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}
        if let Some(x) = self.chars.next() {
            let mut text = String::new();
            text.push(x);
            match x {
                '(' =>  Some(Token {kind: TokenKind::OpenParen, text}),
                ')' =>  Some(Token {kind: TokenKind::CloseParen, text}),
                ',' =>  Some(Token {kind: TokenKind::Comma, text}),
                '=' =>  Some(Token {kind: TokenKind::Equals, text}),
                _ => {
                    if !x.is_alphanumeric() {
                        todo!("Report unexpected token properly starts with '{}'", x);
                    }
                    while let Some(x) = self.chars.next_if(|x| x.is_alphanumeric()) {
                        text.push(x)
                    }
                    Some(Token { kind: TokenKind::Sym, text })
                }
            }
        } else {
            None
        }
    }
}


// Not needed in Example_07.
macro_rules! sym {
  ($name: ident) => {
      Sym(stringify!($name).to_string())
  };
}

// Not needed in Example_07.
macro_rules! fun {
  ($name: ident) => {
      Fun(stringify!($name).to_string(), vec![])
  };
  // args is a repeating group of tokens with a comma sparator. Like Regex * is repeating type.
  ($name: ident, $($args:expr),*) => {
      Fun(stringify!($name).to_string(), vec![$($args),*])
  };
}

macro_rules! fun_args {
  () => { vec![] };
  ($name: ident) => { vec![expr!($name)] };
  ($name: ident, $($rest: tt)*) => {
      {
          let mut t = vec![expr!($name)];
          t.append(&mut fun_args!($($rest)*));
          t
      }
  };
  ($name: ident($($args: tt)*)) => {
      vec![expr!($name($($args)*))]
  };
  ($name: ident($($args: tt)*), $($rest: tt)*) => {
      {
          let mut t = vec![expr!($name($($args)*))];
          t.append(&mut fun_args!($($rest)*));
          t
      }
  }
}

macro_rules! expr {
  ($name: ident) => {
      Sym(stringify!($name).to_string())
  };
  ($name: ident($($args: tt)*)) => {
      Fun(stringify!($name).to_string(), fun_args!($($args)*))
  };
}

pub fn main() {    
  let mut line = String::new();
  let mut confirm = true;
  while confirm {
      print!("Please enter the example number to run: ");  
      stdout().flush().unwrap();
      if line != "" {
          line.pop();
          line.pop();
      }  
      let input = io::stdin().read_line(&mut line);
      let choice: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
      match choice {
          Some(n) if n == 1 => example_01(),
          Some(n) if n == 2 => example_02(),
          Some(n) if n == 3 => example_03(),
          Some(n) if n == 4 => example_04(),
          Some(n) if n == 5 => example_05(),
          Some(n) if n == 6 => example_06(),
          Some(n) if n == 7 => example_07(),
          Some(n) if n == 8 => example_08(),
          Some(_) => println!("Error"),
          None => {},
      }
      line.pop();
      line.pop();
      print!("\nDo you want to try another? (Y/N): ");
      stdout().flush().unwrap();
      io::stdin().read_line(&mut line).unwrap();
      confirm = "y".to_string() == line.replace('\n', "").to_lowercase().to_string();
  }
}

fn example_01() {
  // use Expr::*;
  // swap(pair(a, b)) = pair(b, a)
  let swap = Rule {
      head: Fun(
          "swap".to_string(), 
          vec![Fun("pair".to_string(), 
          vec![Sym("a".to_string()), Sym("b".to_string())])]),
      body: Fun("pair".to_string(), 
          vec![Sym("b".to_string()), Sym("a".to_string())]),
  };

  let expr = Fun(
      "swap".to_string(), 
      vec![Fun("pair".to_string(), 
          vec![Fun("f".to_string(), vec![Sym("a".to_string())]), 
              Fun("g".to_string(), vec![Sym("b".to_string())])])]
  );
  
  // Pattern: swap(pair(a, b))
  let pattern = swap.head.clone();
  println!("Pattern: {}", pattern);
  // Value:   swap(pair(f(c), g(d)))
  let value = Fun(
      "swap".to_string(), 
      vec![
          Fun(
              "pair".to_string(), 
              vec![
                  Fun("f".to_string(), 
                  vec![Sym("c".to_string())]), 
                  Fun("g".to_string(), 
                  vec![Sym("d".to_string())])])
                  ]
              );
  println!("Value:   {}", value);
  // Bindings:
  // a => f(c)
  // b => g(d)
  println!("{:?}", pattern_match(&pattern, &value));
}

fn example_02() {
  // use Expr::*;
  // swap(pair(a, b)) = pair(b, a)
  let swap = Rule {
      head: Fun(
          "swap".to_string(), 
          vec![Fun("pair".to_string(), 
          vec![Sym("a".to_string()), Sym("b".to_string())])]),
      body: Fun("pair".to_string(), 
          vec![Sym("b".to_string()), Sym("a".to_string())]),
  };

  let expr = Fun(
      "swap".to_string(), 
      vec![Fun("pair".to_string(), 
          vec![Fun("f".to_string(), vec![Sym("a".to_string())]), 
              Fun("g".to_string(), vec![Sym("b".to_string())])])]
  );
  
  // Pattern: swap(pair(a, b))
  let pattern = Fun(
      "foo".to_string(), 
      vec![Sym("x".to_string()), 
      Sym("x".to_string())]
  );
  println!("Pattern: {}", pattern);
  // Value:   swap(pair(f(c), g(d)))
  let value = Fun("foo".to_string(), vec![Sym("a".to_string()), Sym("a".to_string())]);
  println!("Value:   {}", value);
  // Bindings:
  // a => f(c)
  // b => g(d)
  if let Some(bindings) = pattern_match(&pattern, &value) {
      println!("Match!");
      for (key, value) in bindings.iter() {
          println!("{} => {}", key, value);
      }
  } else {
      println!("No Match!");
  }
}

fn example_03() {
  // use Expr::*;
  // swap(pair(a, b)) = pair(b, a)
  let swap = Rule {
      head: Fun(
          "swap".to_string(), 
          vec![Fun("pair".to_string(), 
          vec![Sym("a".to_string()), Sym("b".to_string())])]),
      body: Fun("pair".to_string(), 
          vec![Sym("b".to_string()), Sym("a".to_string())]),
  };

  let expr = Fun("foo".to_string(),
      vec![Fun(
          "swap".to_string(), 
          vec![Fun("pair".to_string(), 
              vec![Fun("f".to_string(), vec![Sym("a".to_string())]), 
                  Fun("g".to_string(), vec![Sym("b".to_string())])])]
      ), 
      Fun(
          "swap".to_string(), 
          vec![Fun("pair".to_string(), 
              vec![Fun("q".to_string(), vec![Sym("c".to_string())]), 
                  Fun("z".to_string(), vec![Sym("d".to_string())])])]
      )]
  );

  println!("Rule: {}", swap);
  println!("Expr: {}", expr);
  println!("Expr: {}", swap.apply_all(&expr));
}

fn example_04() {
  for token in Lexer::from_iter("swap(pair(a, b)) = pair(b, a)".chars()) {
      println!("{:?}", token);
  }
  let tokens: Vec<Token> = Lexer::from_iter("swap ( pair(a, b))".chars()).collect();
  println!("\n{:?}", tokens);
  println!("\n{:?}", Expr::parse(Lexer::from_iter("swap ( pair(a, b))".chars())));
  println!("\n{}", Expr::parse(Lexer::from_iter("swap ( pair(a, b))".chars())));
  let source = "swap(pair(a, b))";
  let swap = Rule {
      head: expr!(swap(pair(a, b))),
      body: expr!(pair(b, a)),
  };
  println!("\n{}", swap.apply_all(&Expr::parse(Lexer::from_iter(source.chars()))));
}

fn example_05() {
  println!("{:?}", sym!(a));
  println!("{:?}", Sym("a".to_string()));
  println!("{:?}", fun!(f, sym!(a)));
  println!("{:?}", fun!(f, sym!(a), sym!(b)));
  println!("{}", fun!(f, sym!(a), sym!(b)));
  println!("{}", fun!(f));
  println!("{:?}", expr!(a));
  println!("{:?}", expr!(f()));
  println!("{:?}", expr!(f(a)));
  println!("{:?}", expr!(f(a, b)));
  println!("{:?}", expr!(f(g(b))));
  println!("{:?}", expr!(f(g(b), k(b))));
  println!("{:?}", expr!(f(f(a), b, g())));
}

fn example_06() {
  // swap(pair(a, b)) = pair(b, a)
  let swap = Rule {
      head: fun!(swap, fun!(pair, sym!(a), sym!(b))),
      body: fun!(pair, sym!(b), sym!(a)),
  };

  let expr = fun!(foo, 
      fun!(swap, fun!(pair, fun!(f, sym!(a)), fun!(g, sym!(b)))), 
      fun!(swap, fun!(pair, fun!(q, sym!(c)), fun!(z, sym!(d)))
      )
  );

  println!("Rule: {}", swap);
  println!("Expr: {}", expr);
  println!("Expr: {}", swap.apply_all(&expr));
}

fn example_07() {
  // swap(pair(a, b)) = pair(b, a)
  let swap = Rule {
      head: expr!(swap(pair(a, b))),
      body: expr!(pair(b, a)),
  };

  let expr = expr!{
      foo(
          swap(pair(f(a), g(b))),
          swap(pair(q(c), z(d)))
      )
  };

  println!("Rule: {}", swap);
  println!("Expr: {}", expr);
  println!("Expr: {}", swap.apply_all(&expr));
}

fn example_08() {
  let swap = Rule {
      head: expr!(swap(pair(a, b))),
      body: expr!(pair(b, a)),
  };
  let mut command = String::new();
  let quit = false;
  while !quit {
      command.clear();
      print!("noq> ");
      stdout().flush().unwrap();
      stdin().read_line(&mut command).unwrap();
      if command.to_string().replace("\n", "") == "quit".to_string() {
         break
      }
      // println!("{}", Expr::parse(Lexer::from_iter(command.chars())));
      println!("{}", swap.apply_all(&Expr::parse(Lexer::from_iter(command.chars()))));
  }
}