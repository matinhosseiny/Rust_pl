extern crate let_lang_proj;

use let_lang_proj::let_lang_scanner::*;
use let_lang_proj::let_lang_parser::*;
use let_lang_proj::let_lang_exp::*;
use let_lang_proj::let_lang_env::*;
use let_lang_proj::int_bool::*;

fn value_of(ast: &LetLangExp, env: &LetLangEnv) -> Option<IntBool> { // defined in int_bool.rs
    match ast.clone() {
        LetLangExp::ConstExp(int)    => Some(IntBool::Integer(int)),
        LetLangExp::Boolean(b)       => Some(IntBool::Boolean(b)),
        LetLangExp::DiffExp(e1, e2)  => value_of_diff_exp(&(*e1), &(*e2), env),
        LetLangExp::IsZeroExp(e)     => value_of_iszero(&(*e), env),
        LetLangExp::IfExp(e1,e2,e3)  => value_of_if(&(*e1),&(*e2),&(*e3), env),
        LetLangExp::VarExp(s)        => env.apply_env(&s),
        LetLangExp::LetExp(s,e1,e2)  => value_of_let(&s, &(*e1), &(*e2), env),
    }}

fn value_of_let(s: &String, e1: &LetLangExp, e2: &LetLangExp, env: &LetLangEnv) -> Option<IntBool>{
    let new_val = value_of(e1, env);
    if new_val.is_none() { return None};
    let new_env = env.extend_env(s, new_val.unwrap());
    value_of(e2, &new_env)
}

fn value_of_if(e1: &LetLangExp, e2: &LetLangExp, e3: &LetLangExp, env: &LetLangEnv) -> Option<IntBool> {
    if match value_of(e1, env) { // compute value of test and treat as true only if boolean true
        Some(x) => match x {
            IntBool::Integer(_i) => false,
            IntBool::Boolean(b) => b,
            },
        None => false,
        }
        {
            value_of(e2, env)
        } else {
            value_of(e3, env)
        }}

fn value_of_iszero(e: &LetLangExp, env: &LetLangEnv) -> Option<IntBool> {
    let opt_val = value_of(e, env);
    match opt_val {
        Some(x) => match x {
            IntBool::Integer(i)  => Some(IntBool::Boolean(i == 0)),
            IntBool::Boolean(_b) => None,
        },
        None    => None,
    }}

// checked difference
fn value_of_diff_exp(arg1: &LetLangExp, arg2: &LetLangExp, env: &LetLangEnv) -> Option<IntBool> {
    let val1 = value_of(arg1, env);
    let val2 = value_of(arg2, env);
    if val1.is_none() || val2.is_none() {
        None
    } else {
        let v1 = val1.unwrap();
        let v2 = val2.unwrap();
        Some(IntBool::Integer(value_of_diff_exp_work(&v1, &v2)))
    }}

fn value_of_diff_exp_work(a1: &IntBool, a2: &IntBool) -> i32 {
    let a1_int_val: i32 = match *a1 {
                            IntBool::Integer(i) => i,
                            _                   => 0,
                            };
    let a2_int_val: i32 = match *a2 {
                            IntBool::Integer(i) => i,
                            _                   => 0,
                            };
    a1_int_val - a2_int_val
}

#[allow(dead_code)]
fn main() {
    let str1 = "-(24, +31)";
    let result: Result<Vec<Token>, LexErr> = tokenize(str1);

    match result {
        Ok(v)  => println!("{:?}", v),
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    let str2 = "if true then 1 else -1";
    let result = tokenize(str2);  // leave out type declaration
    match result {
        Ok(v)  => println!("{:?}", v),
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    let str3 = "let temp = 3 in -(temp, 103)";
    match tokenize(str3) {
        Ok(v)  => println!("{:?}", v),
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    let str4 = "if iszero(TextId) then let x = -571 in false";
    match tokenize(str4) {
        Ok(v)  => println!("{:?}", v),
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    let e1 = LetLangExp::new_const_exp(64);
    println!("\ne1: {}                                   (ConstExp)", e1);

    let e2 = LetLangExp::new_diff_exp(&e1, &e1);
    println!("e2: {}                            (DiffExp)", e2);

    let e3 = LetLangExp::new_iszero(&e1);
    println!("e3: {}                           (IsZeroExp)", e3);

    let e4 = LetLangExp::new_if_exp(&e3, &e1, &e2);
    println!("e4: {} (IfExp)", e4);

    let e5 = LetLangExp::new_var_exp(&("myVar".to_string()));
    println!("e5: {}                                (VarExp)", e5);

    let e6 = LetLangExp::new_let_exp(&("x".to_string()), &e1, &e5);
    println!("e6: {}                  (LetExp)", e6);

    println!("\nStarting to parse: str1.");
    let t1 = tokenize(str1).unwrap();
    let ast1_result = parse(&t1);
    match ast1_result {
        Ok(v)  => println!("{}", v),
//        Ok(v)  => println!("{:?}", v),
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    println!("\nStarting to parse: milestone let");
    let mile_str =
    "let x = 7
     in let y = 2
        in let y = let x = -(x, 1)
                   in -(x, y)
           in -(-(x, 8), y)";

    // compute tokens
    let tok_result = tokenize(mile_str);        // returns Result<Vec<Token>, LexErr>
    let mile_tokens = tok_result.unwrap();

    // compute AST and evaluate
    let env = LetLangEnv::EmptyEnv;
    let mile_ast_result = parse(&mile_tokens);  // returns Result<LetLangExp, ParseErr>
    match mile_ast_result {
        Ok(v)  => {println!("{}", v);      // regular print
                   println!("{:#?}", v);   // pretty-print in debug format
                   println!("\nmilestone = {:?}", value_of(&v, &env));},
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    println!("\nStarting to parse IFFFFFFFFFFFFFF");
    let if_str =
    "if iszero(-(x, 11))
    then -(y, 2)
    else -(y, 4)";

    // Create an environment were variables x and y are defined
    let env1 = env.extend_env(&("y".to_string()), IntBool::Integer(22));
    let env2 = env1.extend_env(&("x".to_string()), IntBool::Integer(33));

    // compute and print tokens
    let if_tok_result = tokenize(if_str);
    let if_tokens = if_tok_result.unwrap();
    println!("If tokens: {:?}", if_tokens);  // print in debug format

    let if_ast_result = parse(&if_tokens);
    match if_ast_result {
        Ok(v)  => {println!("{}", v);
                   println!("{:#?}", v);
                   println!("\nif_value = {:?}", value_of(&v, &env2));},
        Err(e) => println!("Syntax error: {:#?}", e),
    }

    println!("\nNow testing: ungrammatical input!");
    let ungram_str =
    "let x = 21 in minus)";
    let if_tok_result = tokenize(ungram_str);
    let if_tokens = if_tok_result.unwrap();
    let if_ast_result = parse(&if_tokens);
    match if_ast_result {
        Ok(v)  => println!("{}", v),
        Err(e) => println!("Syntax error: {:#?}", e),
    }
}
