use std::io;
#[derive(Debug, Clone)]
enum Expr {
    Cons(Box<Expr>, Box<Expr>),
    Atom(Atom),
    Nil,
    T
}

#[derive(Debug, Clone)]
enum Atom {
//    I32(i32),
//    Text(String),
    Symbol(String),
}

fn COND(expr: Box<Expr>)-> Box<Expr> {
    match *expr {
        Expr::Cons(car, cdr) => {
            let eval_cond = eval(car);
            match *eval_cond {
                Expr::T => eval(CADR(cdr)),
                _ => Box::new(Expr::Nil)
            }
        },
        _ => Box::new(Expr::Atom(Atom::Symbol(String::from("Error. Element is not a list")))),
    }
}

fn COND2(expr1: Box<Expr>, expr2: Box<Expr>)-> Box<Expr> {
    match *expr1 {
        Expr::Cons(car, cdr) => {
            let eval_cond = eval(car);
            match *eval_cond {
                Expr::T => eval(CADR(cdr)),
                _ => COND(expr2)
            }
        },
        _ => Box::new(Expr::Atom(Atom::Symbol(String::from("Error. Element is not a list")))),
    }
}

fn ATOM(expr: Box<Expr>)-> Box<Expr> {
    match *expr {
        Expr::Atom(_atom) => Box::new(Expr::T),
        _ => Box::new(Expr::Nil)
    }
}

fn EQ(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    let eval_a = eval(a);
    let eval_b = eval(b);
    match *eval_a {
        Expr::Atom(atom_a) => {
            match atom_a {
                Atom::Symbol(value_a) => {
                    match *eval_b {
                        Expr::Atom(atom_b) => {
                            match atom_b {
                                Atom::Symbol(value_b) => {
                                    if value_a == value_b {
                                        Box::new(Expr::T)
                                    } else {
                                        Box::new(Expr::Nil)
                                    }
                                },
                                _ => Box::new(Expr::Nil)
                            }                           
                        },
                        _ => Box::new(Expr::Nil)
                    }
                }
            }        
        },
        _ => Box::new(Expr::Nil)
    }
}

fn CAR(expr: Box<Expr>)-> Box<Expr> {
    match *expr {
        Expr::Atom(_atom) => Box::new(Expr::Atom(Atom::Symbol(String::from("Error. Element is not a list")))),
        Expr::Cons(car, _cdr) => {
            car
        },
        _ => Box::new(Expr::Nil)
    }
    
}

fn CDR(expr: Box<Expr>)-> Box<Expr> {
    match *expr {
        Expr::Atom(_atom) => Box::new(Expr::Atom(Atom::Symbol(String::from("Error. Element is not a list")))),
        Expr::Cons(_car, cdr) => {
            cdr
        },
        _ => Box::new(Expr::Nil)
    }
    
}

fn CADR(expr: Box<Expr>)-> Box<Expr> {
    CAR(CDR(expr))
}

fn CAAR(expr: Box<Expr>) -> Box<Expr> {
    CAR(CAR(expr))
}

fn CADAR(expr: Box<Expr>) -> Box<Expr> {
    CAR(CDR(CAR(expr)))
}

fn ASSOC(x: Box<Expr>, y: Box<Expr>) -> Box<Expr> {
    let x2 = Box::new((*x).clone());
    let y2 = Box::new((*y).clone());
    let y3 = Box::new((*y).clone());
    COND2(Box::new(Expr::Cons(EQ(CAAR(y), x), CADAR(y2))),
    Box::new(Expr::Cons(Box::new(Expr::T), ASSOC(x2, CDR(y3)))))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    println!("your input: {}", input);
}

fn eval(e: Box<Expr>, a: Box<Expr>) -> Box<Expr> {
    match *input {
        Expr::Atom(atom) => {
            ASSOC(e, a)
        },
        Expr::Cons(car, _cdr) => {
            match car {
                Expr::Atom(atom) => {
                    match atom {
                        Atom::Symbol(symbol) => {
                            if symbols == "quote" {
                                CADR(e)
                            }
                        }
                    }
                },
                Expr:Cons(car1, cdr1) => {
                    match car1 {
                        Expr::Atom {
                            
                        },
                        _ => Box::new(Expr::Nil)
                    }
                },
                _ => Box::new(Expr::Nil)
            }
        },
        Expr::Nil => Box::new(Expr::Nil),
        Expr::T => Box::new(Expr::Nil),
    }
}
