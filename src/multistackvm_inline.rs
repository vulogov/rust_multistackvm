use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    pub fn register_inline(&mut self, name: String, fun: VMInlineFn) -> Result<&mut VM, Error> {
        match self.unregister_inline(format!("{}_inline", &name)) {
            Ok(_) => {
                self.inline_fun.insert(format!("{}_inline", &name), fun);
            }
            Err(err) => {
                bail!("VM Inline unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_inline(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.inline_fun.contains_key(&name) {
            self.inline_fun.remove(&name);
        }
        Ok(self)
    }

    pub fn is_inline(&mut self, name: String) -> bool {
        if self.inline_fun.contains_key(&format!("{}_inline", &name)) {
            return true;
        }
        false
    }

    pub fn get_inline(&mut self, name: String) -> Result<VMInlineFn, Error> {
        if self.inline_fun.contains_key(&format!("{}_inline", &name)) {
            return match self.inline_fun.get(&format!("{}_inline", &name)) {
                Some(fun) => Ok(*fun),
                None => bail!("VM Inline {} is registered, but not found.", &name),
            };
        }
        bail!("VM Inline {} not registered", &name);
    }

    pub fn i_direct(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.is_inline(name.clone()) {
            match self.get_inline(name.clone()) {
                Ok(fun) => {
                    return fun(self);
                }
                Err(err) => {
                    bail!("i({}) returned: {}", &name, err);
                }
            }
        } else {
            match self.stack.get_inline(name.clone()) {
                Ok(fun) => {
                    match fun(&mut self.stack) {
                        Ok(_) => {
                            return Ok(self);
                        }
                        Err(err) => {
                            bail!("VM inline function returned error: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("i({}) for stack returned: {}", &name, err);
                }
            }
        }
    }

    pub fn i(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.is_alias(name.clone()) {
            match self.get_alias(name.clone()) {
                Ok(real_name) => {
                    return self.i_direct(real_name);
                }
                Err(err) => {
                    bail!("Alias resolution for {} returned: {}", &name, err);
                }
            }
        } else {
            return self.i_direct(name.clone());
        }
    }

}
