extern crate regex;
use regex::Regex;
use std::io::BufRead;
// use std::io;
#[derive(Debug, Clone)]
enum Expr {
    Cons(Box<Expr>, Box<Expr>),
    Symbol(String),
    Number(i32),
    Nil,
    T,
}

fn atom(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Cons(_head, _tail) => Box::new(Expr::Nil),
        _ => Box::new(Expr::T),
    }
}

fn eq(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    match *a {
        Expr::Symbol(value_a) => match *b {
            Expr::Symbol(value_b) => {
                if value_a == value_b {
                    Box::new(Expr::T)
                } else {
                    Box::new(Expr::Nil)
                }
            }
            _ => Box::new(Expr::Nil),
        },
        Expr::Number(num_a) => match *b {
            Expr::Number(num_b) => {
                if num_a == num_b {
                    Box::new(Expr::T)
                } else {
                    Box::new(Expr::Nil)
                }
            }
            _ => Box::new(Expr::Nil),
        },
        Expr::T => match *b {
            Expr::T => Box::new(Expr::T),
            _ => Box::new(Expr::Nil),
        },
        Expr::Nil => match *b {
            Expr::Nil => Box::new(Expr::T),
            _ => Box::new(Expr::Nil),
        },
        Expr::Cons(head_a, tail_a) => match *b {
            Expr::Cons(head_b, tail_b) => {
                let eq_head = eq(head_a, head_b);
                match *eq_head {
                    Expr::T => eq(tail_a, tail_b),
                    _ => Box::new(Expr::Nil),
                }
            }
            _ => Box::new(Expr::Nil),
        },
    }
}

fn cons(expr1: Box<Expr>, expr2: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Cons(expr1, expr2))
}

fn car(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Cons(car, _cdr) => car,
        _ => {
            println!("Error. {:?} is not a list", print_car(expr));
            Box::new(Expr::Nil)
        }
    }
}

fn cdr(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Cons(_car, cdr) => cdr,
        _ => {
            println!("Error. {:?} is not a list", print_car(expr));
            Box::new(Expr::Nil)
        }
    }
}

fn cadr(expr: Box<Expr>) -> Box<Expr> {
    car(cdr(expr))
}

fn caar(expr: Box<Expr>) -> Box<Expr> {
    car(car(expr))
}

fn cadar(expr: Box<Expr>) -> Box<Expr> {
    car(cdr(car(expr)))
}

fn caddr(expr: Box<Expr>) -> Box<Expr> {
    car(cdr(cdr(expr)))
}

fn assoc(x: Box<Expr>, env: Box<Expr>) -> Box<Expr> {
    let x2 = Box::new((*x).clone());
    match *env {
        Expr::Cons(first, tail) => match *first {
            Expr::Cons(f, t) => {
                let equal = eq(x, f);
                match *equal {
                    Expr::T => car(t),
                    _ => assoc(x2, tail),
                }
            }
            _ => {
                println!("Error. {:?} is not a list", print_car(first));
                Box::new(Expr::Nil)
            }
        },
        Expr::Nil => Box::new(Expr::Nil),
        _ => {
            println!("Error. {:?} is not a list", print_car(env));
            Box::new(Expr::Nil)
        }
    }
}

fn evcond(expr: Box<Expr>, env: Box<Expr>) -> Box<Expr> {
    let cond = eval(caar(expr.clone()), env.clone()).0;
    match *cond {
        Expr::Nil => evcond(cdr(expr), env),
        _ => eval(cadar(expr), env).0,
    }
}

//fn apply<F>(f: F, list: Box<Expr>) -> Box<Expr>
//where F: Fn(Box<Expr>, Box<Expr>) -> Box<Expr>{
//    match *list {
//        Expr::Nil => Box::new(Expr::Number(0)),
//        Expr::Cons(head, tail) => {
//            println!("plus head: {:?}", head);
//            f.clone()(head, apply(f, tail))
//        },
//        _ => Box::new(Expr::Symbol(String::from("Error. Element is not a list"))),
//    }
//}

fn plus(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    match *a {
        Expr::Number(value_a) => match *b {
            Expr::Number(value_b) => Box::new(Expr::Number(value_a + value_b)),
            _ => {
                println!("Error. {:?} is not a number", print_car(b));
                Box::new(Expr::Nil)
            }
        },
        _ => {
            println!("Error. {:?} is not a number", print_car(a));
            Box::new(Expr::Nil)
        }
    }
}

fn list(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    cons(a, cons(b, Box::new(Expr::Nil)))
}

fn append(x: Box<Expr>, y: Box<Expr>) -> Box<Expr> {
    match *x {
        Expr::Cons(head, tail) => cons(head, append(tail, y)),
        Expr::Nil => y,
        _ => cons(x, y),
    }
}

fn pair(x: Box<Expr>, y: Box<Expr>) -> Box<Expr> {
    match *x {
        Expr::Cons(head_x, tail_x) => match *y {
            Expr::Cons(head_y, tail_y) => cons(list(head_x, head_y), pair(tail_x, tail_y)),
            Expr::Nil => Box::new(Expr::Nil),
            _ => {
                println!("Error. {:?} is not a list", print_car(y));
                Box::new(Expr::Nil)
            }
        },
        Expr::Nil => Box::new(Expr::Nil),
        _ => {
            println!("Error. {:?} is not a list", print_car(x));
            Box::new(Expr::Nil)
        }
    }
}

fn evlis(arguments: Box<Expr>, env: Box<Expr>) -> Box<Expr> {
    match *arguments {
        Expr::Cons(head, tail) => cons(eval(head, env.clone()).0, evlis(tail, env)),
        Expr::Nil => Box::new(Expr::Nil),
        _ => {
            println!("Error. {:?} is not a list", print_car(arguments));
            Box::new(Expr::Nil)
        }
    }
}

fn main() {
    let mut result: (Box<Expr>, Box<Expr>);
    let mut env: Box<Expr> = parse("((hola 10))".to_string());
    loop {
        //       let mut input = String::new();
        //        io::stdin().lock..expect("error reading");
        let input = std::io::stdin();
        for line in input.lock().lines() {
            // here line is a String without the trailing newline
            let parsed = parse(line.unwrap().to_uppercase());
            result = eval(parsed, env);
            env = result.1.clone();
            println!("{}", print_car(result.0));
            println!("env: {}", print_car(result.1));
        }
    }
}

fn eval(expr: Box<Expr>, env: Box<Expr>) -> (Box<Expr>, Box<Expr>) {
    //    let expr2 = Box::new((*expr).clone());
    //    let env2 = Box::new((*env).clone());
    match *expr {
        Expr::T => (Box::new(Expr::T), env),
        Expr::Nil => (Box::new(Expr::Nil), env),
        Expr::Number(num) => (Box::new(Expr::Number(num)), env),
        Expr::Symbol(symbol) => (assoc(Box::new(Expr::Symbol(symbol)), env.clone()), env),
        Expr::Cons(car_elem, cdr_elem) => match *(car_elem.clone()) {
            Expr::Symbol(symbol) => {
                //      println!("your value: {:?}", (symbol).clone());
                if symbol == "QUOTE" {
                    (car(cdr_elem), env)
                } else if symbol == "ATOM" {
                    (atom(eval(car(cdr_elem), env.clone()).0), env)
                } else if symbol == "EQ" {
                    (
                        eq(
                            eval(car(cdr_elem.clone()), env.clone()).0,
                            eval(cadr(cdr_elem), env.clone()).0,
                        ),
                        env,
                    )
                } else if symbol == "CAR" {
                    (car(eval(car(cdr_elem), env.clone()).0), env)
                } else if symbol == "CDR" {
                    (cdr(eval(car(cdr_elem), env.clone()).0), env)
                } else if symbol == "CONS" {
                    (
                        cons(
                            eval(car(cdr_elem.clone()), env.clone()).0,
                            eval(cadr(cdr_elem), env.clone()).0,
                        ),
                        env,
                    )
                } else if symbol == "LIST" {
                    (evlis(cdr_elem, env.clone()), env)
                } else if symbol == "COND" {
                    (evcond(cdr_elem, env.clone()), env)
                } else if symbol == "PLUS" {
                    (
                        plus(
                            eval(car(cdr_elem.clone()), env.clone()).0,
                            eval(cadr(cdr_elem), env.clone()).0,
                        ),
                        env,
                    )
                } else if symbol == "SETQ" {
                    let value = eval(cadr(cdr_elem.clone()), env.clone()).0;
                    let name = car(cdr_elem);
                    (value.clone(), cons(list(name, value), env))
                } else if symbol == "LAMBDA" || symbol == "MACRO" {
                    (cons(car_elem, cdr_elem), env)
                } else if symbol == "EVAL" {
                    eval(eval(car(cdr_elem), env.clone()).0, env)
                } else {
                    (
                        eval(
                            cons(
                                eval(Box::new(Expr::Symbol(symbol)), env.clone()).0,
                                cdr_elem,
                            ),
                            env.clone(),
                        )
                        .0,
                        env,
                    )
                }
            }
            Expr::Cons(car1, cdr1) => match *car1 {
                Expr::Symbol(symbol) => {
                    if symbol == "LABEL" {
                        (
                            eval(
                                cons(cadr(cdr1.clone()), cdr_elem),
                                cons(list(car(cdr1), car_elem), env.clone()),
                            )
                            .0,
                            env,
                        )
                    } else if symbol == "LAMBDA" {
                        (
                            eval(
                                cadr(cdr1.clone()),
                                append(pair(car(cdr1), evlis(cdr_elem, env.clone())), env.clone()),
                            )
                            .0,
                            env,
                        )
                    } else if symbol == "MACRO" {
                        (
                            eval(
                                cadr(cdr1.clone()),
                                append(pair(car(cdr1), cdr_elem), env.clone()),
                            )
                            .0,
                            env,
                        )
                    } else {
                        (
                            eval(
                                cons(
                                    cons(assoc(Box::new(Expr::Symbol(symbol)), env.clone()), cdr1),
                                    cdr_elem,
                                ),
                                env.clone(),
                            )
                            .0,
                            env,
                        )
                    }
                }
                _ => {
                    println!("Error. {:?} is not a symbol", print_car(car1));
                    (Box::new(Expr::Nil), env)
                }
            },
            _ => {
                println!("Error. {:?} is not a symbol", print_car(car_elem));
                (Box::new(Expr::Nil), env)
            }
        },
    }
}

fn parse_s_expr(expr: String) -> Vec<String> {
    let length = expr.len();
    let subexpr = &expr[1..(length - 1)];
    //   println!("subexpr : {:?}", subexpr);

    let paren_regexp = Regex::new(&format!("\\{}|\\{}", "(", ")")).unwrap();
    let mut result = Vec::new();
    let mut pos = 0;
    let mut depth = 0;
    for cap in paren_regexp.find_iter(subexpr) {
        // println!("your value: {:?}", cap);
        let start = cap.start();
        //  println!("your start: {:?}", start);
        let end = cap.end();
        //  println!("your end: {:?}", end);
        let paren = subexpr.chars().nth(start).unwrap();
        //  println!("your paren: {:?}", paren);

        if depth == 0 {
            let first_tokens = subexpr[pos..start]
                .split(char::is_whitespace)
                .filter(|s| !s.is_empty());

            result.extend(first_tokens);
            pos = start;
        }

        if paren == '(' {
            depth += 1;
        } else if paren == ')' {
            depth -= 1;
            if depth == 0 {
                result.push(&subexpr[pos..end]);
                pos = end;
            }
        }
    }
    let final_tokens = subexpr[pos..(subexpr.len())]
        .split(char::is_whitespace)
        .filter(|s| !s.is_empty());

    result.extend(final_tokens);
    //    println!("your result: {:?}", result);
    result.iter().map(|s| s.to_string()).collect()
}

fn is_atom(str: String) -> bool {
    let initLetter = &str[0..1];
    initLetter != "("
}

fn parse_atom(atom: String) -> Box<Expr> {
    let optionInt = atom.parse::<i32>();
    match optionInt {
        Ok(int) => Box::new(Expr::Number(int)),
        Err(_error) => {
            if atom == "T" {
                Box::new(Expr::T)
            } else {
                Box::new(Expr::Symbol(atom))
            }
        }
    }
}

fn parse_list(mut list: Vec<String>) -> Box<Expr> {
    if list.len() == 0 {
        Box::new(Expr::Nil)
    } else {
        let next = list.remove(0);
        //      println!("next: {:?}", next);
        Box::new(Expr::Cons(parse(next), parse_list(list)))
    }
}

fn parse(expr: String) -> Box<Expr> {
    //   println!("expr: {:?}", expr);
    if is_atom(expr.clone()) {
        //    println!("is_atom: {:?}", expr);
        parse_atom(expr)
    } else {
        let elem_list = parse_s_expr(expr);
        parse_list(elem_list)
    }
}

fn print_car(expr: Box<Expr>) -> String {
    match *expr {
        Expr::Symbol(symbol) => symbol,
        Expr::Number(number) => number.to_string(),
        Expr::T => "T".to_string(),
        Expr::Nil => "NIL".to_string(),
        Expr::Cons(head, tail) => "(".to_string() + &print_car(head) + &print_cdr(tail),
    }
}

fn print_cdr(expr: Box<Expr>) -> String {
    match *expr {
        Expr::Symbol(symbol) => symbol,
        Expr::Number(number) => number.to_string(),
        Expr::T => "T".to_string(),
        Expr::Nil => ")".to_string(),
        Expr::Cons(head, tail) => " ".to_string() + &print_car(head) + &print_cdr(tail),
    }
}
