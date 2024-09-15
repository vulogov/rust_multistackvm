use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    pub fn register_command(&mut self, name: String, fun: VMInlineFn) -> Result<&mut VM, Error> {
        match self.unregister_command(name.clone()) {
            Ok(_) => {
                self.command_fun.insert(name, fun);
            }
            Err(err) => {
                bail!("VM Command unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_command(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.command_fun.contains_key(&name) {
            self.command_fun.remove(&name);
        }
        Ok(self)
    }

    pub fn is_command(&mut self, name: String) -> bool {
        if self.command_fun.contains_key(&name) {
            return true;
        }
        false
    }

    pub fn get_command(&mut self, name: String) -> Result<VMInlineFn, Error> {
        if self.command_fun.contains_key(&name) {
            return match self.command_fun.get(&name) {
                Some(fun) => Ok(*fun),
                None => bail!("VM Command {} is registered, but not found.", &name),
            };
        }
        bail!("VM Command {} not registered", &name);
    }

    pub fn c(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.is_command(name.clone()) {
            match self.get_command(name.clone()) {
                Ok(fun) => {
                    return fun(self);
                }
                Err(err) => {
                    bail!("c({}) returned: {}", &name, err);
                }
            }
        }
        Ok(self)
    }

}
