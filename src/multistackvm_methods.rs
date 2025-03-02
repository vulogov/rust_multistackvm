use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    pub fn register_method(&mut self, name: String, fun: VMInlineFn) -> Result<&mut VM, Error> {
        match self.unregister_method(name.clone()) {
            Ok(_) => {
                self.methods_fun.insert(name, fun);
            }
            Err(err) => {
                bail!("VM Method unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_method(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.methods_fun.contains_key(&name) {
            self.methods_fun.remove(&name);
        }
        Ok(self)
    }

    pub fn is_method(&mut self, name: String) -> bool {
        if self.methods_fun.contains_key(&name) {
            return true;
        }
        false
    }

    pub fn get_method(&mut self, name: String) -> Result<VMInlineFn, Error> {
        if self.methods_fun.contains_key(&name) {
            return match self.methods_fun.get(&name) {
                Some(fun) => Ok(*fun),
                None => bail!("VM Method {} is registered, but not found.", &name),
            };
        }
        bail!("VM Method {} not registered", &name);
    }

}
