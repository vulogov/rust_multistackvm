use crate::multistackvm::*;
use easy_error::{Error, bail};

impl VM {
    pub fn to_stack(&mut self, name: String) -> Result<&mut VM, Error> {
        match self.stack.to_stack(name.clone()) {
            Ok(_) => {
                match self.push_stacks(name.clone()) {
                    Ok(_) => {}
                    Err(err) => {
                        bail!("VM::to_stack() pushing stack name returned error: {}", err);
                    }
                }
            }
            Err(err) => {
                bail!("VM::to_stack() returned error: {}", err);
            }
        }
        Ok(self)
    }

}
