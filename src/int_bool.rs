#[derive(Debug,Clone)]

pub enum IntBool {
    Integer(i32),
    Boolean(bool),
}

impl IntBool {
    pub fn to_string(&self) -> String {
        match *self {
            IntBool::Integer(i) => i.to_string(),
            IntBool::Boolean(b) => b.to_string(),
        }
    }
}