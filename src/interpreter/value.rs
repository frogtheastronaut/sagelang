use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::parser::ast::AccessModifier;

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
    Class {
        name: String,
        superclass: Option<Box<Value>>,
        field_access: HashMap<String, AccessModifier>,  // Field name -> access level
        method_access: HashMap<String, AccessModifier>, // Method name -> access level
        methods: HashMap<String, Value>,
    },
    Instance {
        class_name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
        field_access: HashMap<String, AccessModifier>,  // Track field access levels
        methods: HashMap<String, Value>,
        method_access: HashMap<String, AccessModifier>, // Track method access levels
    },
    BoundMethod {
        receiver: Box<Value>,
        method: Box<Value>,
    },
    Null,
}
