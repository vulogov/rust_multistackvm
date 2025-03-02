use crate::multistackvm::*;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use easy_error::{bail, Error};

impl VM {
    pub fn register_class(&mut self, name: String, bclass: Value) -> Result<&mut VM, Error> {
        if ! bclass.is_type(CLASS) {
            bail!("register: argument #1 is not a CLASS");
        }
        match self.unregister_class(name.clone()) {
            Ok(_) => {
                self.classes.insert(name, bclass);
            }
            Err(err) => {
                bail!("VM Class unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_class(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.classes.contains_key(&name) {
            self.classes.remove(&name);
        }
        Ok(self)
    }

    pub fn is_class(&mut self, name: String) -> bool {
        if self.classes.contains_key(&name) {
            return true;
        }
        false
    }

    pub fn get_class(&mut self, name: String) -> Result<Value, Error> {
        if self.classes.contains_key(&name) {
            return match self.classes.get(&name) {
                Some(bclass) => Ok(bclass.clone()),
                None => bail!("VM Class {} is registered, but not found.", &name),
            };
        }
        bail!("VM Class {} not registered", &name);
    }

}
