use std::collections::HashMap;

struct Env(Vec<HashMap<String, i32>>);

impl Env {
    fn new() -> Self {
        Env(Vec::new())
    }

    fn find(&self, name: &str) -> Option<i32> {
        self.0.iter().rev().find_map(|map| map.get(name).copied())
    }

    fn add_frame(&mut self) {
        self.0.push(HashMap::new())
    }

    fn pop_frame(&mut self) {
        self.0.pop();
    }

    fn insert(&mut self, k: String, v: i32) {
        self.0.last_mut().and_then(|map| map.insert(k, v));
    }
}

#[derive(Debug)]
enum Expr {
    List(Vec<Expr>),
    Num(i32),
    Sym(String),
}

use Expr::*;

impl Expr {
    fn evaluate(&self, env: &mut Env) -> i32 {
        match self {
            Num(n) => *n,
            Sym(s) => env.find(s).unwrap(),
            List(l) => Self::eval_list(l, env),
        }
    }

    fn eval_list(list: &[Expr], env: &mut Env) -> i32 {
        let eval_let = |l: &[Expr], env: &mut Env| {
            env.add_frame();
            let pairs = l.chunks_exact(2);
            let tail_expr = &pairs.remainder()[0];
            pairs.for_each(|pair| if let [Sym(s), v] = pair {
                let val = v.evaluate(env);
                env.insert(s.clone(), val);
            });
            let val = tail_expr.evaluate(env);
            env.pop_frame();
            val
        };
        // println!("{:?}", list);
        match list {
            [Sym(s), l @ ..] if s == "let" => eval_let(l, env),
            [Sym(s), a, b] if s == "add" => a.evaluate(env) + b.evaluate(env),
            [Sym(s), a, b] if s == "mult" => a.evaluate(env) * b.evaluate(env),
            _ => unreachable!(),
        }
    }

}

impl Solution {
    fn parse(expr: &str) ->  Expr {
        let parse_atom = |atom: String| atom.parse().map_or(Sym(atom.to_string()), |n| Num(n));
        let mut stack = Vec::new();
        let mut list = Vec::new();
        let mut atom = String::new();
        for c in expr.chars() {
            match c {
                '(' => {
                    stack.push(list);
                    list = Vec::new();
                }
                ' ' => {
                    if !atom.is_empty() {
                        list.push(parse_atom(atom));
                        atom = String::new();
                    }
                }
                ')' => {
                    if !atom.is_empty() {
                        list.push(parse_atom(atom));
                        atom = String::new();
                    }
                    let mut last = stack.pop().unwrap();
                    last.push(List(list));
                    list = last;
                }
                _ => {
                    atom.push(c);
                }
            }
        }
        if atom.is_empty() {
            list.pop().unwrap()
        } else {
            parse_atom(atom)
        }
    }

    pub fn evaluate(expression: String) -> i32 {
        Self::parse(&expression).evaluate(&mut Env::new())
    }
}
