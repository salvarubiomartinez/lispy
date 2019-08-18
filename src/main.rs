extern crate regex;
use std::mem;
use regex::Regex;
use std::io::BufRead;
use std::rc::Rc;

type Expr = Option<Rc<Elem>>;

#[derive(Debug)]
enum Elem {
    T,
    I64(i64),
    Symbol(String),
    Node(Expr, Expr),
}

fn atom(expr: &Expr) -> Expr {
    expr.as_ref().and_then(|elem| match elem.as_ref() {
        Elem::Node(_, _) => None,
        _ => Some(Rc::new(Elem::T)),
    })
}

fn eq(a: &Rc<Expr>, b: &Rc<Expr>) -> Rc<Expr> {
    //    println!("eq a. {:?}", print_car(a));
    //    println!("eq b. {:?}", print_car(b));
    match &**a {
        Expr::Symbol(value_a) => match &**b {
            Expr::Symbol(value_b) => {
                if value_a == value_b {
                    Rc::new(Expr::T)
                } else {
                    Rc::new(Expr::Nil)
                }
            }
            _ => Rc::new(Expr::Nil),
        },
        Expr::Number(num_a) => match &**b {
            Expr::Number(num_b) => {
                if num_a == num_b {
                    Rc::new(Expr::T)
                } else {
                    Rc::new(Expr::Nil)
                }
            }
            _ => Rc::new(Expr::Nil),
        },
        Expr::T => match &**b {
            Expr::T => Rc::new(Expr::T),
            _ => Rc::new(Expr::Nil),
        },
        Expr::Nil => match &**b {
            Expr::Nil => Rc::new(Expr::T),
            _ => Rc::new(Expr::Nil),
        },
        Expr::Cons(head_a, tail_a) => match &**b {
            Expr::Cons(head_b, tail_b) => {
                let eq_head = eq(head_a, head_b);
                match *eq_head {
                    Expr::T => eq(tail_a, tail_b),
                    _ => Rc::new(Expr::Nil),
                }
            }
            _ => Rc::new(Expr::Nil),
        },
    }
}

fn cons(a: &Expr, b: &Expr) -> Expr {
    Some(Rc::new(Elem::Node(a.clone(), b.clone())))
}

fn car(expr: &Expr) -> Expr {
    expr.as_ref().and_then(|elem| match elem.as_ref() {
        Elem::Node(head, _tail) => head.clone(),
        _ => {
            println!("car: Error. {:?} is not a list", expr);
            None
        }
    })
}

fn cdr(expr: &Expr) -> Expr {
    expr.as_ref().and_then(|elem| match elem.as_ref() {
        Elem::Node(_head, tail) => tail.clone(),
        _ => {
            println!("cdr: Error. {:?} is not a list", expr);
            None
        }
    })
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

fn list(a: &Expr, b: &Expr) -> Expr {
    cons(a, &cons(b, &None))
}

fn append(a: &Expr, b: &Expr) -> Expr {
    match a.as_ref() {
        Some(rc) => match rc.as_ref() {
            Elem::Node(head, tail) => cons(head, &append(tail, b)),
            _ => cons(a, b),
        },
        None => b.clone(),
    }
}

fn pair(a: &Expr, b: &Expr) -> Expr {
    a.as_ref().and_then(|elem_a| match elem_a.as_ref() {
        Elem::Node(head_a, tail_a) => b.as_ref().and_then(|elem_b| match elem_b.as_ref() {
            Elem::Node(head_b, tail_b) => cons(&list(head_a, head_b), &pair(tail_a, tail_b)),
            _ => {
                println!("cdr: Error. {:?} is not a list", a);
                None
            }
        }),
        _ => {
            println!("cdr: Error. {:?} is not a list", a);
            None
        }
    })
}


fn assoc(x: &Expr, env: &Expr, global_env: &Expr) -> Expr {
    env.as_ref().and_then(|elem_a| match elem_a.as_ref() {
        Elem::Node(head, tail) => {
            let cond = eq(x, &car(head));
            match cond.as_ref() {
                None => assoc(x, tail, global_env),
                _ => cadr(head),
            } 
        },
        _ => {
            println!("cdr: Error. {:?} is not a list", a);
            None
        }
    })
}

fn evlis(arguments: &Rc<Expr>, env: &Rc<Expr>, global_env: &mut Rc<Expr>) -> Rc<Expr> {
    // println!("evlis arguments: {:?}", print_car(arguments));
    match &**arguments {
        Expr::Cons(head, tail) => cons(&eval(head, env, global_env), &evlis(tail, env, global_env)),
        Expr::Nil => Rc::new(Expr::Nil),
        _ => {
            println!("Error. {:?} is not a list", print_car(&arguments));
            Rc::new(Expr::Nil)
        }
    }
}

fn arithmetic_op(a: &Rc<Expr>, b: &Rc<Expr>, f: &Fn(&i64, &i64) -> i64) -> Rc<Expr> {
    match &**a {
        Expr::Number(value_a) => match &**b {
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

fn evcond(expr: &Rc<Expr>, env: &Rc<Expr>, global_env: &mut Rc<Expr>) -> Rc<Expr> {
    let cond = eval(&caar(expr), env, global_env);
    match *cond {
        Expr::Nil => evcond(&cdr(expr), env, global_env),
        _ => eval(&cadar(expr), env, global_env),
    }
}

fn eval(expr: &Expr, env: &Expr, global_env: &mut Expr) -> Expr {
    expr.as_ref().and_then(|elem| match elem.as_ref() {
        Elem::Symbol(_) => assoc(expr, env, global_env),
        Elem::Node(car_elem, cdr_elem) => {
            car_elem
                .as_ref()
                .and_then(|subelem| match subelem.as_ref() {
                    Elem::Symbol(symbol) => {
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
                            append(
                                &eval(&car(cdr_elem), env, global_env),
                                &eval(&cadr(cdr_elem), env, global_env),
                            )
                        } else {
                            eval(
                                &cons(&eval(car_elem, env, global_env), cdr_elem),
                                env,
                                global_env,
                            )
                        }
                    }
                    Elem::Node(car1, cdr1) => {
                        car1.as_ref()
                            .and_then(|subsubelem| match subsubelem.as_ref() {
                                Elem::Symbol(symbol) => {
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
                                                    &Some(Rc::new(Elem::Symbol(
                                                        "&ARGS".to_string(),
                                                    ))),
                                                    &evlis(cdr_elem, env, global_env),
                                                ),
                                                &append(
                                                    &pair(
                                                        &car(cdr1),
                                                        &evlis(cdr_elem, env, global_env),
                                                    ),
                                                    env,
                                                ),
                                            ),
                                            global_env,
                                        )
                                    } else if symbol == "LOOP" {
                                        //             println!("loop global env. 2 {:?}", print_car(&env));
                                        let mut loop_env = append(
                                            &pair(&car(cdr1), &evlis(cdr_elem, env, global_env)),
                                            env,
                                        );
                                        //             println!("loop global env. 2 {:?}", print_car(&env));
                                        loop {
                                            let cond =
                                                &eval(&caadr(cdr1), &mut loop_env, global_env);
                                            match cond.as_ref() {
                                                None => {
                                                    loop_env = append(
                                                        &pair(
                                                            &car(cdr1),
                                                            &evlis(
                                                                &caddr(cdr1),
                                                                &mut loop_env,
                                                                global_env,
                                                            ),
                                                        ),
                                                        env,
                                                    );
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
                                                &list(
                                                    &Some(Rc::new(Elem::Symbol(
                                                        "&ARGS".to_string(),
                                                    ))),
                                                    cdr_elem,
                                                ),
                                                &append(&pair(&car(cdr1), cdr_elem), env),
                                            ),
                                            global_env,
                                        )
                                    } else {
                                        eval(
                                            &cons(
                                                &cons(
                                                    &assoc(
                                                        &Some(Rc::new(Elem::Symbol(
                                                            symbol.to_string(),
                                                        ))),
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
                                    println!("Error. {:?} is not a function", print_car(car_elem));
                                    None
                                }
                            })
                    }
                    _ => {
                        println!("Error. {:?} is not a function", print_car(car_elem));
                        None
                    }
                })
        }
        _ => expr.clone(),
    })
}

fn main() {
    let env: &mut Rc<Expr>= &mut parse( &("((QUOTE QUOTE) (ATOM ATOM) (EQ EQ) (CAR CAR) (CDR CDR) (CONS CONS) (LIST LIST) (COND COND) (PLUS PLUS) (PROD PROD) (DIFF DIFF) (QUOT) (QUOT) (LAMBDA LAMBDA) (MACRO MACRO) (SETQ SETQ) (APPEND APPEND) (REDUCE REDUCE) (LOOP LOOP) (NIL ()))".to_string()) );
    loop {
        let input = std::io::stdin();
        for line in input.lock().lines() {
            let parsed = parse(&line.unwrap().to_uppercase());
            let result = eval(&parsed, &env.clone(), env);
            println!("{}", print_car(&result));
        }
    }
}

fn print_car(expr: &Expr) -> String {
    match expr.as_ref() {
        Some(rc) => match rc.as_ref() {
            Elem::Symbol(symbol) => symbol.to_string(),
            Elem::I64(number) => number.to_string(),
            Elem::T => "T".to_string(),
            Elem::Node(head, tail) => "(".to_string() + &print_car(head) + &print_cdr(tail),
        },
        None => "NIL".to_string(),
    }
}

fn print_cdr(expr: &Expr) -> String {
    match expr.as_ref() {
        Some(rc) => match rc.as_ref() {
            Elem::Symbol(symbol) => " . ".to_string() + &symbol.to_string() + &")".to_string(),
            Elem::I64(number) => " . ".to_string() + &number.to_string() + &")".to_string(),
            Elem::T => " . ".to_string() + &"T".to_string() + &")".to_string(),
            Elem::Node(head, tail) => " ".to_string() + &print_car(head) + &print_cdr(tail),
        },
        None => ")".to_string(),
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
            if depth == 0 {
                println!("Unmatched open token at {}", pos);
            }
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
    let option_int = atom.parse::<i64>();
    match option_int {
        Ok(int) => Some(Rc::new(Elem::I64(int))),
        Err(_error) => {
            if atom == "NIL" {
                None
            } else if atom == "T" {
                Some(Rc::new(Elem::T))
            } else {
                let init_letter = &atom[0..1];
                let rest_word = &atom[1..(atom.len())];
                if init_letter != "\'" {
                    Some(Rc::new(Elem::Symbol(atom.to_string())))
                } else {
                    list(
                        &Some(Rc::new(Elem::Symbol("QUOTE".to_string()))),
                        &Some(Rc::new(Elem::Symbol(rest_word.to_string()))),
                    )
                }
            }
        }
    }
}

fn parse_list(mut list: Vec<String>) -> Expr {
    if list.len() == 0 {
        None
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
