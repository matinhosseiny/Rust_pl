// Let-language Expressions

use std::rc::Rc; // Rc<T> reference counted pointer type over immutable value
use std::fmt;

// data type for abstract-syntax tree
#[derive(Debug,Clone)]
pub enum LetLangExp {  // set of possible LetLangExp's
    ConstExp(i32),
    Boolean(bool),
    DiffExp(Rc<LetLangExp>, Rc<LetLangExp>),
    IsZeroExp(Rc<LetLangExp>),
    IfExp(Rc<LetLangExp>, Rc<LetLangExp>, Rc<LetLangExp>),
    VarExp(String),
    LetExp(String, Rc<LetLangExp>, Rc<LetLangExp>),
}

// create a constructor and to_string() method for each type of LetLangExp
impl LetLangExp {
    pub fn new_const_exp(num: i32) -> Self {
        LetLangExp::ConstExp(num)
    }
    pub fn new_boolean(tv: bool) -> Self {
        LetLangExp::Boolean(tv)
    }
    pub fn new_diff_exp(arg1: &LetLangExp, arg2: &LetLangExp) -> Self {
        LetLangExp::DiffExp(Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }
    pub fn new_iszero(arg: &LetLangExp) -> Self {
        LetLangExp::IsZeroExp(Rc::new(arg.clone()))
    }
    pub fn new_if_exp(arg1: &LetLangExp, arg2: &LetLangExp, arg3: &LetLangExp) -> Self {
        LetLangExp::IfExp(Rc::new(arg1.clone()), Rc::new(arg2.clone()), Rc::new(arg3.clone()))
    }
    pub fn new_var_exp(s: &String) -> Self {
        LetLangExp::VarExp(s.clone())
    }
    pub fn new_let_exp(s: &String, arg1: &LetLangExp, arg2: &LetLangExp) -> Self {
        LetLangExp::LetExp(s.clone(), Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }
    // a string representation, to be used by the formatter, for each type of LetLangExp
    pub fn to_string(&self) -> String {
        match self.clone() {
            LetLangExp::ConstExp(int)       => int.to_string(),
            LetLangExp::Boolean(bool)       => bool.to_string(),
            LetLangExp::DiffExp(e1, e2)     => {let mut temp: String = "-(".to_string();
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(", ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp.push_str(&(")".to_string()));
                                                temp}
            LetLangExp::IsZeroExp(e)        => {let mut temp = "iszero(".to_string();
                                                temp.push_str(&(e.to_string()));
                                                temp.push_str(&(")".to_string()));
                                                temp}
            LetLangExp::IfExp(e1, e2, e3)   => {let mut temp = "if ".to_string();
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(" then ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp.push_str(&(" else ".to_string()));
                                                temp.push_str(&(e3.to_string()));
                                                temp}
            LetLangExp::VarExp(var)         => var,
            LetLangExp::LetExp(v, e1, e2)   => {let mut temp = "let ".to_string();
                                                temp.push_str(&(v.to_string()));
                                                temp.push_str(&(" = ".to_string()));
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(" in ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp}
        }}
}

impl fmt::Display for LetLangExp { // do not change this code
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("");
        let s1 = self.to_string();
        s.push_str(&s1);
        write!(f, "{}", s)
    }}
