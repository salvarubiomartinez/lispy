use std::io;
#[derive(Debug, Clone)]
enum Expr {
    Cons(Box<Expr>, Box<Expr>),
    Atom(Atom),
    Nil,
    T,
}

#[derive(Debug, Clone)]
enum Atom {
    //    I32(i32),
    //    Text(String),
    Symbol(String),
}

fn cond(expr: Box<Expr>, env: Box<Expr>) -> Box<Expr> {
    let env2 = Box::new((*env).clone());
    match *expr {
        Expr::Cons(car, cdr) => {
            let eval_cond = eval(car, env);
            match *eval_cond {
                Expr::T => eval(cadr(cdr), env2),
                _ => Box::new(Expr::Nil),
            }
        }
        _ => Box::new(Expr::Atom(Atom::Symbol(String::from(
            "Error. Element is not a list",
        )))),
    }
}

fn cond2(expr1: Box<Expr>, expr2: Box<Expr>, env: Box<Expr>) -> Box<Expr> {
    let env2 = Box::new((*env).clone());
    let env3 = Box::new((*env).clone());
    match *expr1 {
        Expr::Cons(car, cdr) => {
            let eval_cond = eval(car, env);
            match *eval_cond {
                Expr::T => eval(cadr(cdr), env2),
                _ => cond(expr2, env3),
            }
        }
        _ => Box::new(Expr::Atom(Atom::Symbol(String::from(
            "Error. Element is not a list",
        )))),
    }
}

fn atom(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Atom(_atom) => Box::new(Expr::T),
        _ => Box::new(Expr::Nil),
    }
}

fn eq(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    match *a {
        Expr::Atom(atom_a) => match atom_a {
            Atom::Symbol(value_a) => match *b {
                Expr::Atom(atom_b) => match atom_b {
                    Atom::Symbol(value_b) => {
                        if value_a == value_b {
                            Box::new(Expr::T)
                        } else {
                            Box::new(Expr::Nil)
                        }
                    }
                },
                _ => Box::new(Expr::Nil),
            },
        },
        _ => Box::new(Expr::Nil),
    }
}

fn cons(expr1: Box<Expr>, expr2: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Cons(expr1, expr2))
}

fn car(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Atom(_atom) => Box::new(Expr::Atom(Atom::Symbol(String::from(
            "Error. Element is not a list",
        )))),
        Expr::Cons(car, _cdr) => car,
        _ => Box::new(Expr::Nil),
    }
}

fn cdr(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Atom(_atom) => Box::new(Expr::Atom(Atom::Symbol(String::from(
            "Error. Element is not a list",
        )))),
        Expr::Cons(_car, cdr) => cdr,
        _ => Box::new(Expr::Nil),
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
    let env2 = Box::new((*env).clone());
    let env3 = Box::new((*env).clone());
    let env4 = Box::new((*env).clone());
    let env5 = Box::new((*env).clone());

    match *env {
        Expr::Cons(first, tail) => match *first {
            Expr::Cons(f, t) => {
                let equal = eq(x, f);
                match *equal {
                    Expr::T => car(t),
                    _ => assoc(x2, tail),
                }
            }
            _ => Box::new(Expr::Atom(Atom::Symbol(String::from(
                "Error. Element is not a list",
            )))),
        },
        Expr::Nil => Box::new(Expr::Nil),
        _ => Box::new(Expr::Atom(Atom::Symbol(String::from(
            "Error. Element is not a list",
        )))),
    }
}

fn main() {
    let value = assoc(
        Box::new(Expr::Atom(Atom::Symbol(String::from("hola")))),
        Box::new(Expr::Cons(
            Box::new(Expr::Cons(
                Box::new(Expr::Atom(Atom::Symbol(String::from("hola")))),
                Box::new(Expr::Cons(
                    Box::new(Expr::Atom(Atom::Symbol(String::from("pepe")))),
                    Box::new(Expr::Nil),
                )),
            )),
            Box::new(Expr::Nil),
        )),
    );
    println!("your value: {:?}", value);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    println!("your input: {}", input);
}

fn eval(expr: Box<Expr>, env: Box<Expr>) -> Box<Expr> {
    let expr2 = Box::new((*expr).clone());
    let expr3 = Box::new((*expr).clone());
    let expr4 = Box::new((*expr).clone());
    let expr5 = Box::new((*expr).clone());
    let expr6 = Box::new((*expr).clone());
    let expr7 = Box::new((*expr).clone());
    let expr8 = Box::new((*expr).clone());
    let expr9 = Box::new((*expr).clone());
    let expr10 = Box::new((*expr).clone());
    let env2 = Box::new((*env).clone());
    let env3 = Box::new((*env).clone());
    let env4 = Box::new((*env).clone());
    let env5 = Box::new((*env).clone());
    let env6 = Box::new((*env).clone());
    let env7 = Box::new((*env).clone());
    let env8 = Box::new((*env).clone());
    let env9 = Box::new((*env).clone());
    let env10 = Box::new((*env).clone());
    match *expr {
        Expr::Atom(_atom) => assoc(expr2, env),
        Expr::Cons(car_elem, _cdr) => match *car_elem {
            Expr::Atom(elem) => match elem {
                Atom::Symbol(symbol) => {
                    println!("your value: {:?}", (symbol).clone());
                    if symbol == "quote" {
                        cadr(expr2)
                    } else if symbol == "atom" {
                        atom(eval(cadr(expr2), env2))
                    } else if symbol == "eq" {
                        eq(eval(cadr(expr2), env3), eval(caddr(expr3), env4))
                    } else if symbol == "car" {
                        car(eval(cadr(expr2), env6))
                    } else if symbol == "cdr" {
                        cdr(eval(cadr(expr2), env7))
                    } else if symbol == "cons" {
                        cons(eval(cadr(expr2), env8), eval(caddr(expr3), env9))
                    } else {
                        Box::new(Expr::Nil)
                    }
                }
            },
            Expr::Cons(car1, _cdr1) => match *car1 {
                Expr::Atom(_atom) => Box::new(Expr::Nil),
                _ => Box::new(Expr::Nil),
            },
            _ => Box::new(Expr::Nil),
        },
        Expr::Nil => Box::new(Expr::Nil),
        Expr::T => Box::new(Expr::Nil),
    }
}
