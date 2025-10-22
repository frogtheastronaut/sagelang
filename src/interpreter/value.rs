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
        #[allow(dead_code)]
        superclass: Option<Box<Value>>,
        field_access: HashMap<String, AccessModifier>,  // field name -> access level
        method_access: HashMap<String, AccessModifier>, // method name -> access level
        methods: HashMap<String, Value>,                // instance methods
        static_methods: HashMap<String, Value>,         // static methods
    },
    Instance {
        class_name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
        field_access: HashMap<String, AccessModifier>,  // track field access levels
        methods: HashMap<String, Value>,                // instance methods
        method_access: HashMap<String, AccessModifier>, // track method access levels
        static_methods: HashMap<String, Value>,         // static methods from class
    },
    BoundMethod {
        receiver: Box<Value>,
        method: Box<Value>,
    },
    Null,
}
