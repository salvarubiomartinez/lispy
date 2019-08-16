extern crate regex;
use std::rc::Rc;
use regex::Regex;
use std::io::BufRead;
use std::mem;
// use std::io;
#[derive(Debug)]
enum Expr {
    Cons(Rc<Expr>, Rc<Expr>),
    Symbol(String),
    Number(i32),
    Nil,
    T,
}

fn atom(expr: &Expr) -> Rc<Expr> {
    match expr {
        Expr::Cons(_head, _tail) => Rc::new(Expr::Nil),
        _ => Rc::new(Expr::T),
    }
}
>
fn eq(a: Rc<Expr>, b: Rc<Expr>) -> Rc<Expr> {
    //    println!("eq a. {:?}", print_car(a.clone()));
    //    println!("eq b. {:?}", print_car(b.clone()));
    match &*a {
        Expr::Symbol(value_a) => match &*b {
            Expr::Symbol(value_b) => {
                if value_a == value_b {
                    Rc::new(Expr::T)
                } else {
                    Rc::new(Expr::Nil)
                }
            }
            _ => Rc::new(Expr::Nil),
        },
        Expr::Number(num_a) => match &*b {
            Expr::Number(num_b) => {
                if num_a == num_b {
                    Rc::new(Expr::T)
                } else {
                    Rc::new(Expr::Nil)
                }
            }
            _ => Rc::new(Expr::Nil),
        },
        Expr::T => match &*b {
            Expr::T => Rc::new(Expr::T),
            _ => Rc::new(Expr::Nil),
        },
        Expr::Nil => match &*b {
            Expr::Nil => Rc::new(Expr::T),
            _ => Rc::new(Expr::Nil),
        },
        Expr::Cons(head_a, tail_a) => match &*b {
            Expr::Cons(head_b, tail_b) => {
                let eq_head = eq(head_a, head_b);
                match &*eq_head {
                    Expr::T => eq(tail_a, tail_b),
                    _ => Rc::new(Expr::Nil),
                }
            }
            _ => Rc::new(Expr::Nil),
        },
    }
}

fn not(expr: Rc<Expr>) -> Rc<Expr> {
    match &*expr {
        Expr::Nil => Rc::new(Expr::T),
        _ => Rc::new(Expr::Nil),
    }
}

fn cons(expr1: Rc<Expr>, expr2: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::Cons(expr1, expr2))
}

fn car(expr: Rc<Expr>) -> Rc<Expr> {
    match &*expr {
        Expr::Cons(car, _cdr) => car,
        _ => {
            println!("Error. {:?} is not a list", print_car(&expr));
            Rc::new(Expr::Nil)
        }
    }
}

fn cdr(expr: Rc<Expr>) -> Rc<Expr> {
    match &*expr {
        Expr::Cons(_car, cdr) => cdr,
        _ => {
            println!("Error. {:?} is not a list", print_car(&expr));
            Rc::new(Expr::Nil)
        }
    }
}

fn cadr(expr: Rc<Expr>) -> Rc<Expr> {
    car(cdr(expr))
}

fn caar(expr: Rc<Expr>) -> Rc<Expr> {
    car(car(expr))
}

fn cadar(expr: Rc<Expr>) -> Rc<Expr> {
    car(cdr(car(expr)))
}

fn caddr(expr: Rc<Expr>) -> Rc<Expr> {
    car(cdr(cdr(expr)))
}

fn caadr(expr: Rc<Expr>) -> Rc<Expr> {
    car(car(cdr(expr)))
}

fn cadadr(expr: Rc<Expr>) -> Rc<Expr> {
    car(cdr(car(cdr(expr))))
}

fn assoc(x: Rc<Expr>, env: Rc<Expr>) -> Rc<Expr> {
    let x2 = Rc::new((*x).clone());
    match &*env {
        Expr::Cons(first, tail) => match &*first {
            Expr::Cons(f, t) => {
                let equal = eq(x, f);
                match &*equal {
                    Expr::T => car(t),
                    _ => assoc(x2, tail),
                }
            }
            _ => {
                println!("Error. {:?} is not a list", print_car(&first));
                Rc::new(Expr::Nil)
            }
        },
        Expr::Nil => {
            println!("Error. {:?} is undefined", print_car(&x));
            Rc::new(Expr::Nil)
        }
        _ => {
            println!("Error. {:?} is not a list", print_car(&env));
            Rc::new(Expr::Nil)
        }
    }
}

fn evcond(expr: Rc<Expr>, env: &Rc<Expr>, global_env: &mut Rc<Expr>) -> Rc<Expr> {
    let cond = eval(caar(expr.clone()), env, global_env);
    match &*cond {
        Expr::Nil => evcond(cdr(expr), env, global_env),
        _ => eval(cadar(expr), env, global_env),
    }
}

//fn apply<F>(f: F, list: Rc<Expr>) -> Rc<Expr>
//where F: Fn(Rc<Expr>, Rc<Expr>) -> Rc<Expr>{
//    match &*list {
//        Expr::Nil => Rc::new(Expr::Number(0)),
//        Expr::Cons(head, tail) => {
//            println!("plus head: {:?}", head);
//            f.clone()(head, apply(f, tail))
//        },
//        _ => Rc::new(Expr::Symbol(String::from("Error. Element is not a list"))),
//    }
//}

fn plus(a: Rc<Expr>, b: Rc<Expr>) -> Rc<Expr> {
    match &*a {
        Expr::Number(value_a) => match &*b {
            Expr::Number(value_b) => Rc::new(Expr::Number(value_a + value_b)),
            _ => {
                println!("Error. {:?} is not a number", print_car(&b));
                Rc::new(Expr::Nil)
            }
        },
        _ => {
            println!("Error. {:?} is not a number", print_car(&a));
            Rc::new(Expr::Nil)
        }
    }
}

fn arithmetic_op(a: Rc<Expr>, b: Rc<Expr>, f: &Fn(i32, i32) -> i32) -> Rc<Expr> {
    match &*a {
        Expr::Number(value_a) => match &*b {
            Expr::Number(value_b) => Rc::new(Expr::Number(f(value_a, value_b))),
            _ => {
                println!("Error. {:?} is not a number", print_car(&b));
                Rc::new(Expr::Nil)
            }
        },
        _ => {
            println!("Error. {:?} is not a number", print_car(&a));
            Rc::new(Expr::Nil)
        }
    }
}

fn list(a: Rc<Expr>, b: Rc<Expr>) -> Rc<Expr> {
    cons(a, cons(b, Rc::new(Expr::Nil)))
}

fn append(x: Rc<Expr>, y: Rc<Expr>) -> Rc<Expr> {
    //   let mut acc = y;
    //   let mut ls = x;
    //   loop {
    //       match &*ls {
    //           Expr::Cons(head, tail) => {
    //               ls = cdr(ls);
    //           },

    //           Expr::Nil => {
    //               break;
    //           }
    //       }
    //   }
    //   acc
    match &*x {
        Expr::Cons(head, tail) => cons(head, append(tail, y)),
        Expr::Nil => y,
        _ => cons(x, y),
    }
}

fn pair(x: Rc<Expr>, y: Rc<Expr>) -> Rc<Expr> {
    match &*x {
        Expr::Cons(head_x, tail_x) => match &*y {
            Expr::Cons(head_y, tail_y) => cons(list(head_x, head_y), pair(tail_x, tail_y)),
            Expr::Nil => Rc::new(Expr::Nil),
            _ => {
                println!("Error. {:?} is not a list", print_car(&y));
                Rc::new(Expr::Nil)
            }
        },
        Expr::Nil => Rc::new(Expr::Nil),
        _ => {
            println!("Error. {:?} is not a list", print_car(&x));
            Rc::new(Expr::Nil)
        }
    }
}

fn evlis(arguments: Rc<Expr>, env: &Rc<Expr>, global_env: &mut Rc<Expr>) -> Rc<Expr> {
    match &*arguments {
        Expr::Cons(head, tail) => cons(eval(head, env, global_env), evlis(tail, env, global_env)),
        Expr::Nil => Rc::new(Expr::Nil),
        _ => {
            println!("Error. {:?} is not a list", print_car(&arguments));
            Rc::new(Expr::Nil)
        }
    }
}

fn main() {
    let mut result: Rc<Expr>;
    let env: &mut Rc<Expr> = &mut parse("((QUOTE QUOTE) (ATOM ATOM) (EQ EQ) (CAR CAR) (CDR CDR) (CONS CONS) (LIST LIST) (COND COND) (PLUS PLUS) (PROD PROD) (DIFF DIFF) (QUOT) (QUOT) (LAMBDA LAMBDA) (MACRO MACRO) (SETQ SETQ) (APPEND APPEND) (REDUCE REDUCE) (LOOP LOOP) (NIL ()))".to_string());
    loop {
        //       let mut input = String::new();
        //        io::stdin().lock..expect("error reading");
        let input = std::io::stdin();
        for line in input.lock().lines() {
            // here line is a String without the trailing newline
            let parsed = parse(line.unwrap().to_uppercase());
            result = eval(parsed, &env.clone(), env);
            println!("{}", print_car(&result));
            // println!("env: {}", print_car(env));
        }
    }
}

fn eval(expr: Rc<Expr>, env: &Rc<Expr>, global_env: &mut Rc<Expr>) -> Rc<Expr> {
    //    let expr2 = Rc::new((*expr).clone());
    //    let env2 = Rc::new((*env).clone());
    match &*expr {
        Expr::T => Rc::new(Expr::T),
        Expr::Nil => Rc::new(Expr::Nil),
        Expr::Number(num) => Rc::new(Expr::Number(num)),
        Expr::Symbol(symbol) => assoc(Rc::new(Expr::Symbol(symbol)), (*env).clone()),

        Expr::Cons(car_elem, cdr_elem) => match &*(car_elem.clone()) {
            Expr::Symbol(symbol) => {
                //      println!("your value: {:?}", (symbol).clone());
                if symbol == "QUOTE" {
                    car(cdr_elem)
                } else if symbol == "ATOM" {
                    atom(eval(car(cdr_elem), env, global_env))
                } else if symbol == "EQ" {
                    eq(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                    )
                } else if symbol == "CAR" {
                    car(eval(car(cdr_elem), env, global_env))
                } else if symbol == "CDR" {
                    cdr(eval(car(cdr_elem), env, global_env))
                } else if symbol == "CONS" {
                    cons(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                    )
                } else if symbol == "LIST" {
                    evlis(cdr_elem, env, global_env)
                } else if symbol == "COND" {
                    evcond(cdr_elem, env, global_env)
                } else if symbol == "PLUS" {
                    arithmetic_op(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                        &|a, b| a + b,
                    )
                } else if symbol == "PROD" {
                    arithmetic_op(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                        &|a, b| a * b,
                    )
                } else if symbol == "DIFF" {
                    arithmetic_op(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                        &|a, b| a - b,
                    )
                } else if symbol == "QUOT" {
                    arithmetic_op(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                        &|a, b| a / b,
                    )
                } else if symbol == "SETQ" {
                    let value = eval(cadr(cdr_elem.clone()), env, global_env);
                    let name = car(cdr_elem);
                    // println!("setq value {:?}", print_car(&value));
                    // global_env = &cons(list(name, value.clone()), (*global_env).clone());
                    mem::swap(
                        global_env,
                        &mut cons(list(name, value.clone()), (*global_env).clone()),
                    );
                    value
                } else if symbol == "LAMBDA" || symbol == "MACRO" {
                    cons(car_elem, cdr_elem)
                } else if symbol == "EVAL" {
                    let new_context = eval(car(cdr_elem), env, global_env);
                    let resp = eval(new_context, env, global_env);
                    //  println!("eval resp {:?}", print_car(&resp));
                    //  println!("eval env {:?}", print_car(&resp));
                    resp
                } else if symbol == "APPEND" {
                    append(
                        eval(car(cdr_elem.clone()), env, global_env),
                        eval(cadr(cdr_elem), env, global_env),
                    )
                } else if symbol == "REDUCE" {
                    let f = eval(car(cdr_elem.clone()), env, global_env);
                    // println!("reduce f {:?}", print_car(&f));
                    let mut ls = eval(cadr(cdr_elem.clone()), env, global_env);
                    // println!("reduce ls {:?}", print_car(&ls));
                    let mut acc = eval(caddr(cdr_elem), env, global_env);
                    // println!("reduce acc {:?}", print_car(&acc));
                    //    while (not(eq(ls, Rc::new(Expr::Nil)))) {\
                    loop {
                        match &*ls {
                            Expr::Cons(current, rest) => {
                                let exp =
                                    cons(f.clone(), cons(current, cons(acc, Rc::new(Expr::Nil))));
                                //         println!("reduce loop exp {:?}", print_car(&exp));

                                acc = eval(exp, env, global_env);
                                //        println!("reduce loop acc {:?}", print_car(&acc));
                                ls = rest;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    acc
                } else {
                    eval(
                        cons(
                            eval(Rc::new(Expr::Symbol(symbol)), env, global_env),
                            cdr_elem,
                        ),
                        env,
                        global_env,
                    )
                }
            }
            Expr::Cons(car1, cdr1) => match &*car1 {
                Expr::Symbol(symbol) => {
                    if symbol == "LABEL" {
                        eval(
                            cons(cadr(cdr1.clone()), cdr_elem),
                            &cons(list(car(cdr1), car_elem), (*env).clone()),
                            global_env,
                        )
                    } else if symbol == "LAMBDA" {
                        eval(
                            cadr(cdr1.clone()),
                            &cons(
                                list(
                                    Rc::new(Expr::Symbol("&ARGS".to_string())),
                                    evlis(cdr_elem.clone(), env, global_env),
                                ),
                                append(
                                    pair(car(cdr1), evlis(cdr_elem, env, global_env)),
                                    (*env).clone(),
                                ),
                            ),
                            global_env,
                        )
                    } else if symbol == "LOOP" {
                        let mut loop_env = append(
                            pair(car(cdr1.clone()), evlis(cdr_elem, env, global_env)),
                            (*env).clone(),
                        );
                        loop {
                            let cond = eval(caadr(cdr1.clone()), &loop_env, global_env);
                            match &*cond {
                                Expr::Nil => {
                                    loop_env = append(
                                        pair(
                                            car(cdr1.clone()),
                                            evlis(caddr(cdr1.clone()), &loop_env, global_env),
                                        ),
                                        (*env).clone(),
                                    );
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        eval(cadadr(cdr1), &loop_env, global_env)
                    } else if symbol == "MACRO" {
                        eval(
                            cadr(cdr1.clone()),
                            &cons(
                                list(
                                    Rc::new(Expr::Symbol("&ARGS".to_string())),
                                    cdr_elem.clone(),
                                ),
                                append(pair(car(cdr1), cdr_elem), (*env).clone()),
                            ),
                            global_env,
                        )
                    } else {
                        eval(
                            cons(
                                cons(assoc(Rc::new(Expr::Symbol(symbol)), env.clone()), cdr1),
                                cdr_elem,
                            ),
                            env,
                            global_env,
                        )
                    }
                }
                _ => {
                    println!("Error. {:?} is not a symbol", print_car(&car1));
                    Rc::new(Expr::Nil)
                }
            },
            _ => {
                println!("Error. {:?} is not a symbol", print_car(&car_elem));
                Rc::new(Expr::Nil)
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
    let init_letter = &str[0..1];
    init_letter != "("
}

fn parse_atom(atom: String) -> Rc<Expr> {
    let option_int = atom.parse::<i32>();
    match option_int {
        Ok(int) => Rc::new(Expr::Number(int)),
        Err(_error) => {
            if atom == "NIL" {
                Rc::new(Expr::Nil)
            } else if atom == "T" {
                Rc::new(Expr::T)
            } else {
                let init_letter = &atom[0..1];
                let rest_word = &atom[1..(atom.len())];
                if init_letter != "\'" {
                    Rc::new(Expr::Symbol(atom))
                } else {
                    list(
                        Rc::new(Expr::Symbol("QUOTE".to_string())),
                        Rc::new(Expr::Symbol(rest_word.to_string())),
                    )
                }
            }
        }
    }
}

fn parse_list(mut list: Vec<String>) -> Rc<Expr> {
    if list.len() == 0 {
        Rc::new(Expr::Nil)
    } else {
        let next = list.remove(0);
        //      println!("next: {:?}", next);
        Rc::new(Expr::Cons(parse(next), parse_list(list)))
    }
}

fn parse(expr: String) -> Rc<Expr> {
    //   println!("expr: {:?}", expr);
    if is_atom(expr.clone()) {
        //    println!("is_atom: {:?}", expr);
        parse_atom(expr)
    } else {
        let elem_list = parse_s_expr(expr);
        parse_list(elem_list)
    }
}

fn print_car(expr: &Expr) -> String {
    match expr {
        Expr::Symbol(symbol) => symbol.to_string(),
        Expr::Number(number) => number.to_string(),
        Expr::T => "T".to_string(),
        Expr::Nil => "NIL".to_string(),
        Expr::Cons(head, tail) => "(".to_string() + &print_car(head) + &print_cdr(tail),
    }
}

fn print_cdr(expr: &Expr) -> String {
    match expr {
        Expr::Symbol(symbol) => symbol.to_string(),
        Expr::Number(number) => number.to_string(),
        Expr::T => "T".to_string(),
        Expr::Nil => ")".to_string(),
        Expr::Cons(head, tail) => " ".to_string() + &print_car(head) + &print_cdr(tail),
    }
}
