use crate::multistackvm::*;
use easy_error::{Error};

impl VM {
    pub fn call_internal_word(&mut self, name: String) -> Result<&mut VM, Error> {
        let fun_name = &name[1..];
        self.i(fun_name.to_string().clone())
    }
}
