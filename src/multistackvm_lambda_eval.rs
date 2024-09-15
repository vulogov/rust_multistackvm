use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    pub fn lambda_eval(&mut self, value: Value) -> Result<&mut VM, Error> {
        match value.dt {
            LAMBDA => {
                match value.cast_lambda() {
                    Ok(lambda_content) => {
                        for v in lambda_content {
                            match self.apply(v) {
                                Ok(_) => {}
                                Err(err) => {
                                    bail!("Lambda content evaluation returned error: {}", err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        bail!("Can not get the lambda body: {}", err);
                    }
                }
            }
            _ => {
                bail!("This is not a lambda");
            }
        }
        Ok(self)
    }
}
