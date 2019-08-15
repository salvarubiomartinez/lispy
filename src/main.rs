extern crate regex;
use regex::Regex;
use std::io::BufRead;
use std::mem;
// use std::io;
#[derive(Debug, Clone)]
enum Expr {
    Cons(Box<Expr>, Box<Expr>),
    Symbol(String),
    Number(i32),
    Nil,
    T,
}

fn atom(expr: &Expr) -> Expr {
    match expr {
        Expr::Cons(_head, _tail) => Expr::Nil,
        _ => Expr::T,
    }
}

fn eq(a: &Expr, b: &Expr) -> Expr {
    //    println!("eq a. {:?}", print_car(a));
    //    println!("eq b. {:?}", print_car(b));
    match a {
        Expr::Symbol(value_a) => match b {
            Expr::Symbol(value_b) => {
                if value_a == value_b {
                    Expr::T
                } else {
                    Expr::Nil
                }
            }
            _ => Expr::Nil,
        },
        Expr::Number(num_a) => match b {
            Expr::Number(num_b) => {
                if num_a == num_b {
                    Expr::T
                } else {
                    Expr::Nil
                }
            }
            _ => Expr::Nil,
        },
        Expr::T => match b {
            Expr::T => Expr::T,
            _ => Expr::Nil,
        },
        Expr::Nil => match b {
            Expr::Nil => Expr::T,
            _ => Expr::Nil,
        },
        Expr::Cons(head_a, tail_a) => match b {
            Expr::Cons(head_b, tail_b) => {
                let eq_head = eq(head_a, head_b);
                match eq_head {
                    Expr::T => eq(tail_a, tail_b),
                    _ => Expr::Nil,
                }
            }
            _ => Expr::Nil,
        },
    }
}

fn not(expr: &Expr) -> Expr {
    match expr {
        Expr::Nil => Expr::T,
        _ => Expr::Nil,
    }
}

fn cons(expr1: &Expr, expr2: &Expr) -> Expr {
    let head = Box::new(expr1.clone());
    let tail = Box::new(expr2.clone());

    Expr::Cons(head, tail)
}

fn car(expr: &Expr) -> Expr {
    match expr {
        Expr::Cons(car, _cdr) => *(car.clone()),
        Expr::Nil => Expr::Nil,
        _ => {
            println!("Error. {:?} is not a list", print_car(&expr));
            Expr::Nil
        }
    }
}

fn cdr(expr: &Expr) -> Expr {
    match expr {
        Expr::Cons(_car, cdr) => *(cdr.clone()),
        Expr::Nil => Expr::Nil,
        _ => {
            println!("Error. {:?} is not a list", print_car(&expr));
            Expr::Nil
        }
    }
}

fn cadr(expr: &Expr) -> Expr {
    car(&cdr(expr))
}

fn caar(expr: &Expr) -> Expr {
    car(&car(expr))
}

fn cadar(expr: &Expr) -> Expr {
    car(&cdr(&car(expr)))
}

fn caddr(expr: &Expr) -> Expr {
    car(&cdr(&cdr(expr)))
}

fn caadr(expr: &Expr) -> Expr {
    car(&car(&cdr(expr)))
}

fn cadadr(expr: &Expr) -> Expr {
    car(&cdr(&car(&cdr(expr))))
}

fn assoc(x: &Expr, env: &mut Expr, global_env: &mut Expr) -> Expr {
    let cond = eq(x, &caar(env));
    match cond {
        Expr::Nil => assoc(x, &mut cdr(env), global_env),
        _ => cadar(env),
    }

    //    let x2 = Box::new((*x));
    //    match env {
    //        Expr::Cons(first, tail) => match first {
    //            Expr::Cons(f, t) => {
    //                let equal = eq(x, f);
    //                match equal {
    //                    Expr::T => car(t),
    //                    _ => assoc(x2, tail),
    //                }
    //            }
    //            _ => {
    //                println!("Error. {:?} is not a list", print_car(&first));
    //                Expr::Nil
    //            }
    //        },
    //        Expr::Nil => {
    //            println!("Error. {:?} is undefined", print_car(&x));
    //            Expr::Nil
    //        }
    //        _ => {
    //            println!("Error. {:?} is not a list", print_car(&env));
    //            Expr::Nil
    //        }
    //    }
}

fn evcond(expr: &Expr, env: &mut Expr, global_env: &mut Expr) -> Expr {
    let cond = eval(&caar(expr), env, global_env);
    match cond {
        Expr::Nil => evcond(&cdr(expr), env, global_env),
        _ => eval(&cadar(expr), env, global_env),
    }
}

//fn apply<F>(f: F, list: &Expr) -> Expr
//where F: Fn(Box<Expr>, Box<Expr>) -> Expr{
//    match list {
//        Expr::Nil => Box::new(Expr::Number(0)),
//        Expr::Cons(head, tail) => {
//            println!("plus head: {:?}", head);
//            f(head, apply(f, tail))
//        },
//        _ => Box::new(Expr::Symbol(String::from("Error. Element is not a list"))),
//    }
//}

fn plus(a: &Expr, b: &Expr) -> Expr {
    match a {
        Expr::Number(value_a) => match b {
            Expr::Number(value_b) => Expr::Number(value_a + value_b),
            _ => {
                println!("Error. {:?} is not a number", print_car(&b));
                Expr::Nil
            }
        },
        _ => {
            println!("Error. {:?} is not a number", print_car(&a));
            Expr::Nil
        }
    }
}

fn arithmetic_op(a: &Expr, b: &Expr, f: &Fn(&i32, &i32) -> i32) -> Expr {
    match a {
        Expr::Number(value_a) => match b {
            Expr::Number(value_b) => Expr::Number(f(value_a, value_b)),
            _ => {
                println!("Error. {:?} is not a number", print_car(&b));
                Expr::Nil
            }
        },
        _ => {
            println!("Error. {:?} is not a number", print_car(&a));
            Expr::Nil
        }
    }
}

fn list(a: &Expr, b: &Expr) -> Expr {
    cons(a, &cons(b, &Expr::Nil))
}

fn append_fn(x: &mut Expr, y: &mut Expr) {
    let mut ls: &mut Expr = x;
    loop {
        match ls {
            Expr::Cons(_head, tail) => {
                ls = tail;
            }

            Expr::Nil => {
                let mut temp = y;
                mem::swap(ls, &mut temp);
                break;
            }
            _ => {
                println!("Error. {:?} is not a list", print_car(ls));
            }
        }
    }
}

fn append(x: &Expr, y: &Expr) -> Expr {
    match x {
        Expr::Cons(head, tail) => cons(head, &append(tail, y)),
        _ => cons(x, y),
    }
}

fn pair(x: &Expr, y: &Expr) -> Expr {
    match x {
        Expr::Cons(head_x, tail_x) => match y {
            Expr::Cons(head_y, tail_y) => cons(&list(head_x, head_y), &pair(tail_x, tail_y)),
            Expr::Nil => Expr::Nil,
            _ => {
                println!("Error. {:?} is not a list", print_car(&y));
                Expr::Nil
            }
        },
        Expr::Nil => Expr::Nil,
        _ => {
            println!("Error. {:?} is not a list", print_car(&x));
            Expr::Nil
        }
    }
}

fn evlis(arguments: &Expr, env: &mut Expr, global_env: &mut Expr) -> Expr {
    match arguments {
        Expr::Cons(head, tail) => cons(&eval(head, env, global_env), &evlis(tail, env, global_env)),
        Expr::Nil => Expr::Nil,
        _ => {
            println!("Error. {:?} is not a list", print_car(&arguments));
            Expr::Nil
        }
    }
}

fn main() {
    let mut result: Expr;
    let env: &mut Expr = &mut parse( &("((QUOTE QUOTE) (ATOM ATOM) (EQ EQ) (CAR CAR) (CDR CDR) (CONS CONS) (LIST LIST) (COND COND) (PLUS PLUS) (PROD PROD) (DIFF DIFF) (QUOT) (QUOT) (LAMBDA LAMBDA) (MACRO MACRO) (SETQ SETQ) (APPEND APPEND) (REDUCE REDUCE) (LOOP LOOP) (NIL ()))".to_string()) );
    loop {
        //       let mut input = String::new();
        //        io::stdin().lock..expect("error reading");
        let input = std::io::stdin();
        for line in input.lock().lines() {
            // here line is a String without the trailing newline
            let parsed = parse(&line.unwrap().to_uppercase());
            result = eval(&parsed, &mut env.clone(), env);
            println!("{}", print_car(&result));
            // println!("env: {}", print_car(env));
        }
    }
}

fn eval(expr: &Expr, env: &mut Expr, global_env: &mut Expr) -> Expr {
    //    let expr2 = Box::new((*expr));
    //    let env2 = Box::new((*env));
    match expr {
        Expr::T => Expr::T,
        Expr::Nil => Expr::Nil,
        Expr::Number(num) => Expr::Number(*num),
        Expr::Symbol(symbol) => assoc(&Expr::Symbol(symbol.to_string()), env, global_env),
        Expr::Cons(head, cdr_elem) => {
            let car_elem: &Expr = head;
            match car_elem {
                Expr::Symbol(symbol) => {
                    //      println!("your value: {:?}", (symbol));
                    if symbol == "QUOTE" {
                        car(cdr_elem)
                    } else if symbol == "ATOM" {
                        atom(&eval(&car(cdr_elem), env, global_env))
                    } else if symbol == "EQ" {
                        eq(
                            &eval(&car(cdr_elem), env, global_env),
                            &eval(&cadr(cdr_elem), env, global_env),
                        )
                    } else if symbol == "CAR" {
                        car(&eval(&car(cdr_elem), env, global_env))
                    } else if symbol == "CDR" {
                        cdr(&eval(&car(cdr_elem), env, global_env))
                    } else if symbol == "CONS" {
                        cons(
                            &eval(&car(cdr_elem), env, global_env),
                            &eval(&cadr(cdr_elem), env, global_env),
                        )
                    } else if symbol == "LIST" {
                        evlis(cdr_elem, env, global_env)
                    } else if symbol == "COND" {
                        evcond(cdr_elem, env, global_env)
                    } else if symbol == "PLUS" {
                        arithmetic_op(
                            &eval(&car(cdr_elem), env, global_env),
                            &eval(&cadr(cdr_elem), env, global_env),
                            &|a, b| a + b,
                        )
                    } else if symbol == "PROD" {
                        arithmetic_op(
                            &eval(&car(cdr_elem), env, global_env),
                            &eval(&cadr(cdr_elem), env, global_env),
                            &|a, b| a * b,
                        )
                    } else if symbol == "DIFF" {
                        arithmetic_op(
                            &eval(&car(cdr_elem), env, global_env),
                            &eval(&cadr(cdr_elem), env, global_env),
                            &|a, b| a - b,
                        )
                    } else if symbol == "QUOT" {
                        arithmetic_op(
                            &eval(&car(cdr_elem), env, global_env),
                            &eval(&cadr(cdr_elem), env, global_env),
                            &|a, b| a / b,
                        )
                    } else if symbol == "SETQ" {
                        let value = eval(&cadr(cdr_elem), env, global_env);
                        let name = &car(cdr_elem);
                        // println!("setq value {:?}", print_car(&value));
                        // global_env = &cons(list(name, value), (*global_env));
                        mem::swap(global_env, &mut cons(&list(name, &value), global_env));
                        value
                    } else if symbol == "LAMBDA" || symbol == "MACRO" {
                        cons(car_elem, cdr_elem)
                    } else if symbol == "EVAL" {
                        let new_context = &eval(&car(cdr_elem), env, global_env);
                        let resp = eval(new_context, env, global_env);
                        //  println!("eval resp {:?}", print_car(&resp));
                        //  println!("eval env {:?}", print_car(&resp));
                        resp
                    } else if symbol == "APPEND" {
                        let mut x = eval(&car(cdr_elem), env, global_env);
                        let mut y = eval(&cadr(cdr_elem), env, global_env);
                        append_fn(&mut x, &mut y);
                        x
                    } else if symbol == "REDUCE" {
                        let f = eval(&car(cdr_elem), env, global_env);
                        // println!("reduce f {:?}", print_car(&f));
                        let mut ls: &mut Expr = &mut eval(&cadr(cdr_elem), env, global_env);
                        // println!("reduce ls {:?}", print_car(&ls));
                        let mut acc = eval(&caddr(cdr_elem), env, global_env);
                        // println!("reduce acc {:?}", print_car(&acc));
                        //    while (not(eq(ls, Expr::Nil))) {\
                        loop {
                            match ls {
                                Expr::Cons(current, rest) => {
                                    let exp = &cons(&f, &cons(current, &cons(&acc, &Expr::Nil)));
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
                            &cons(
                                &eval(&Expr::Symbol(symbol.to_string()), env, global_env),
                                cdr_elem,
                            ),
                            env,
                            global_env,
                        )
                    }
                }
                Expr::Cons(head1, tail1) => {
                    let car1: &Expr = head1;
                    let cdr1: &Expr = tail1;
                    match car1 {
                        Expr::Symbol(symbol) => {
                            if symbol == "LABEL" {
                                eval(
                                    &cons(&cadr(cdr1), cdr_elem),
                                    &mut cons(&list(&car(cdr1), car_elem), env),
                                    global_env,
                                )
                            } else if symbol == "LAMBDA" {
                                eval(
                                    &cadr(cdr1),
                                    &mut cons(
                                        &list(
                                            &Expr::Symbol("&ARGS".to_string()),
                                            &evlis(cdr_elem, env, global_env),
                                        ),
                                        &append(
                                            &pair(&car(cdr1), &evlis(cdr_elem, env, global_env)),
                                            env,
                                        ),
                                    ),
                                    global_env,
                                )
                            } else if symbol == "LOOP" {
                                //             println!("loop global env. 2 {:?}", print_car(&env));
                                let mut loop_env =
                                    pair(&car(cdr1), &evlis(cdr_elem, env, global_env));
                                //             println!("loop global env. 2 {:?}", print_car(&env));
                                append_fn(&mut loop_env, env);
                                //             println!("loop global env. 2 {:?}", print_car(&env));
                                loop {
                                    let cond: &Expr =
                                        &eval(&caadr(cdr1), &mut loop_env, global_env);
                                    match cond {
                                        Expr::Nil => {
                                            //           println!("loop env. 1 {:?}", print_car(&loop_env));
                                            loop_env = pair(
                                                &car(cdr1),
                                                &evlis(&caddr(cdr1), &mut loop_env, global_env),
                                            );
                                            //         println!("loop env. 2 {:?}", print_car(&loop_env));
                                            //       println!("loop global env. 2 {:?}", print_car(&env));
                                            append_fn(&mut loop_env, env);
                                            //        println!("loop env. 3 {:?}", print_car(&loop_env));
                                            //    loop_env = append(
                                            //        pair(
                                            //            car(cdr1),
                                            //            evlis(caddr(cdr1), &loop_env, global_env),
                                            //        ),
                                            //        (*env),
                                            //    );
                                        }
                                        _ => {
                                            break;
                                        }
                                    }
                                }
                                eval(&cadadr(cdr1), &mut loop_env, global_env)
                            } else if symbol == "MACRO" {
                                eval(
                                    &cadr(cdr1),
                                    &mut cons(
                                        &list(&Expr::Symbol("&ARGS".to_string()), cdr_elem),
                                        &append(&pair(&car(cdr1), cdr_elem), env),
                                    ),
                                    global_env,
                                )
                            } else {
                                eval(
                                    &cons(
                                        &cons(
                                            &assoc(
                                                &Expr::Symbol(symbol.to_string()),
                                                env,
                                                global_env,
                                            ),
                                            cdr1,
                                        ),
                                        cdr_elem,
                                    ),
                                    env,
                                    global_env,
                                )
                            }
                        }
                        _ => {
                            println!("Error. {:?} is not a symbol", print_car(&car1));
                            Expr::Nil
                        }
                    }
                }
                _ => {
                    println!("Error. {:?} is not a symbol", print_car(&car_elem));
                    Expr::Nil
                }
            }
        }
    }
}

fn parse_s_expr(expr: &String) -> Vec<String> {
    let length = expr.len();
    let subexpr = &expr[1..(length - 1)];
    //   println!("subexpr : {:?}", subexpr);

    let paren_regexp = Regex::new(&format!("\\{}|\\{}", "(", ")")).unwrap();
    let mut result = Vec::new();
    let mut pos = 0;
    let mut depth = 0;
    for cap in paren_regexp.find_iter(&subexpr) {
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

fn is_atom(str: &String) -> bool {
    let init_letter = &str[0..1];
    init_letter != "("
}

fn parse_atom(atom: &String) -> Expr {
    let option_int = atom.parse::<i32>();
    match option_int {
        Ok(int) => Expr::Number(int),
        Err(_error) => {
            if atom == "NIL" {
                Expr::Nil
            } else if atom == "T" {
                Expr::T
            } else {
                let init_letter = &atom[0..1];
                let rest_word = &atom[1..(atom.len())];
                if init_letter != "\'" {
                    Expr::Symbol(atom.to_string())
                } else {
                    list(
                        &Expr::Symbol("QUOTE".to_string()),
                        &Expr::Symbol(rest_word.to_string()),
                    )
                }
            }
        }
    }
}

fn parse_list(mut list: Vec<String>) -> Expr {
    if list.len() == 0 {
        Expr::Nil
    } else {
        let next = list.remove(0);
        //      println!("next: {:?}", next);
        cons(&parse(&next), &parse_list(list))
    }
}

fn parse(expr: &String) -> Expr {
    //   println!("expr: {:?}", expr);
    if is_atom(expr) {
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
