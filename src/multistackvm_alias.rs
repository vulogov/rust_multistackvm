use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    pub fn register_alias(&mut self, alias: String, name: String) -> Result<&mut VM, Error> {
        match self.unregister_alias(alias.clone()) {
            Ok(_) => {
                self.name_mapping.insert(alias, format!("{}_inline", &name));
            }
            Err(err) => {
                bail!("VM Alias unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_alias(&mut self, name: String) -> Result<&mut VM, Error> {
        if self.name_mapping.contains_key(&name) {
            self.name_mapping.remove(&name);
        }
        Ok(self)
    }

    pub fn is_alias(&mut self, name: String) -> bool {
        if self.name_mapping.contains_key(&name) {
            return true;
        }
        false
    }

    pub fn get_alias(&mut self, name: String) -> Result<String, Error> {
        if self.name_mapping.contains_key(&name) {
            return match self.name_mapping.get(&name) {
                Some(aname) => Ok(aname.clone()),
                None => bail!("VM Alias {} is registered, but not found.", &name),
            };
        }
        bail!("VM Alias {} not registered", &name);
    }

}
