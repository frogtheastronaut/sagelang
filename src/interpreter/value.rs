#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Function {
        #[allow(dead_code)]
        name: String,
        param_count: usize,
        chunk: crate::vm::Chunk,
    },
    Null,
}
