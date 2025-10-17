use super::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    pub values: HashMap<String, Value>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Env { values: HashMap::new(), parent: None }
    }
    pub fn with_parent(parent: Env) -> Self {
        Env { values: HashMap::new(), parent: Some(Box::new(parent)) }
    }
    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(v) => Some(v.clone()),
            None => self.parent.as_ref().and_then(|p| p.get(name)),
        }
    }
    pub fn set(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}
