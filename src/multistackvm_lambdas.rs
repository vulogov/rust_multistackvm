use crate::multistackvm::*;
use rust_dynamic::value::Value;
use easy_error::{bail, Error};

impl VM {
    pub fn register_lambda(&mut self, name: String, lambda: Value) -> Result<&mut VM, Error> {
        match self.unregister_lambda(name.clone()) {
            Ok(_) => {
                self.lambdas.insert(name, lambda);
            }
            Err(err) => {
                bail!("VM Lambda unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_lambda(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.lambdas.contains_key(&name) {
            self.lambdas.remove(&name);
        }
        Ok(self)
    }

    pub fn is_lambda(&mut self, name: String) -> bool {
        if self.lambdas.contains_key(&name) {
            return true;
        }
        false
    }

    pub fn get_lambda(&mut self, name: String) -> Result<Value, Error> {
        if self.lambdas.contains_key(&name) {
            return match self.lambdas.get(&name) {
                Some(lambda) => Ok(lambda.clone()),
                None => bail!("VM Lambda {} is registered, but not found.", &name),
            };
        }
        bail!("VM Lambda {} not registered", &name);
    }

}
