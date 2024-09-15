use rust_dynamic::value::Value;
use crate::multistackvm::*;
use easy_error::{Error};

impl VM {
    pub fn call(&mut self, name: String) -> Result<&mut VM, Error> {
        self.apply(Value::call(name.clone(), Vec::new()))
    }
    pub fn call_in(&mut self, name: String, stack_name: String) -> Result<&mut VM, Error> {
        self.apply_in(stack_name.clone(), Value::call(name.clone(), Vec::new()))
    }
}
