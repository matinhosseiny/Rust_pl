use std::rc::Rc;
use std::fmt;
use int_bool::*;

#[derive(Debug,Clone)]
pub enum LetLangEnv {
    EmptyEnv,
    ExtendEnv(String, IntBool, Rc<LetLangEnv>),
}

impl LetLangEnv {
    pub fn new_env() -> Self {
        LetLangEnv::EmptyEnv
    }
    pub fn extend_env(&self, s:&String, val: IntBool) -> Self {
        LetLangEnv::ExtendEnv(s.clone(), val, Rc::new(self.clone()))
    }
    pub fn apply_env(&self, s:&String) -> Option<IntBool> {
        match self.clone() {
            LetLangEnv::ExtendEnv(var, val, env) =>
                                       if s[..] == var[..] {
                                        Some(val)
                                       } else {
                                        env.apply_env(s)
                                        },
            LetLangEnv::EmptyEnv => None,
        }}
    pub fn is_null_env(&self) -> bool {
        match self.clone() {
            LetLangEnv::EmptyEnv  => true,
            _                     => false,
        }}
    pub fn to_string(&self) -> String {
        match self.clone() {
            LetLangEnv::EmptyEnv => "[]".to_string(),
            LetLangEnv::ExtendEnv(var,val,env) => {let mut temp = "[".to_string();
                                                temp.push_str(&(var.to_string()));
                                                temp.push_str(&(", ".to_string()));
                                                temp.push_str(&(val.to_string()));
                                                temp.push_str(&(" ".to_string()));
                                                temp.push_str(&(env.to_string()));
                                                temp.push_str(&("]".to_string()));
                                                temp},
        }}
}

impl fmt::Display for LetLangEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("");
        let s1 = self.to_string();
        s.push_str(&s1);
        write!(f, "{}", s)
    }}

#[cfg(test)]
mod test {
    use super::LetLangEnv;
    use let_lang_exp::*;

    #[test]
    fn basic_tests() {
        let null_env = LetLangEnv::new_env();
        assert!(null_env.is_null_env());

        let env2 = null_env.extend_env(&("var1".to_string()), 25);
        assert!(!(env2.is_null_env()));
    }
}
