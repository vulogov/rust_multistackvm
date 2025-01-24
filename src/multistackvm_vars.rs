use crate::multistackvm::*;
use rust_dynamic::value::Value;
use std::collections;
use easy_error::{bail, Error};

impl VM {
    pub fn ensure_var_namespace(&mut self, name: String) -> bool {
        if self.vars.contains_key(&name) {
            return true;
        }
        let _ = self.vars.insert(name, collections::HashMap::new());
        return false;
    }
    pub fn register_var(&mut self, name: String, value: Value) -> Result<&mut VM, Error> {
        let s_name = match self.stack.current_stack_name() {
            Some(s_name) => s_name,
            None => bail!("Can not detect current stack name"),
        };
        self.ensure_var_namespace(s_name.clone());
        match self.vars.get_mut(&s_name) {
            Some(n_table) => {
                let _ = n_table.insert(name, value);
            }
            None => bail!("VAR namespace {} is missed", &name),
        }
        Ok(self)
    }

    pub fn unregister_var(&mut self, name: String) -> Result<&mut VM, Error> {
        let s_name = match self.stack.current_stack_name() {
            Some(s_name) => s_name,
            None => bail!("Can not detect current stack name"),
        };
        self.ensure_var_namespace(s_name.clone());
        match self.vars.get_mut(&s_name) {
            Some(n_table) => {
                let _ = n_table.remove(&name);
            }
            None => bail!("VAR namespace {} is missed", &name),
        }
        Ok(self)
    }

    pub fn is_var(&mut self, name: String) -> bool {
        let s_name = match self.stack.current_stack_name() {
            Some(s_name) => s_name,
            None => return false,
        };
        self.ensure_var_namespace(s_name.clone());
        match self.vars.get_mut(&s_name) {
            Some(n_table) => {
                if n_table.contains_key(&name) {
                    return true;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    pub fn get_var(&mut self, name: String) -> Result<Value, Error> {
        let s_name = match self.stack.current_stack_name() {
            Some(s_name) => s_name,
            None => bail!("Can not detect current stack name"),
        };
        self.ensure_var_namespace(s_name.clone());
        match self.vars.get_mut(&s_name) {
            Some(n_table) => {
                match n_table.get(&name) {
                    Some(value) => return Ok(value.clone()),
                    None => bail!("VAR not regustered: {}", &name),
                };
            }
            None => bail!("VAR namespace {} is missed", &name),
        }
    }
}
