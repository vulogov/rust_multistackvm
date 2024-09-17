use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    pub fn apply_in(&mut self, name: String, value: Value) -> Result<&mut VM, Error> {
        match value.dt {
            CALL => {
                match value.cast_string() {
                    Ok(fun_name) => {
                        if self.is_command(fun_name.clone()) {
                            return self.c(fun_name.clone());
                        } else {
                            if self.autoadd {
                                self.stack.push_to_stack(name.clone(), value);
                            } else {
                                if self.is_lambda(fun_name.clone()) {
                                    match self.get_lambda(fun_name.clone()) {
                                        Ok(lambda) => {
                                            return self.lambda_eval_in(name.clone(), lambda);
                                        }
                                        Err(err) => {
                                            bail!("Error getting lambda {}: {}", &fun_name, err);
                                        }
                                    }
                                } else {
                                    return self.i(fun_name.clone());
                                }
                            }
                        }
                    }
                    Err(err) => {
                        bail!("Can not get the name of function the CALL value: {}", err);
                    }
                }
            }
            CONTEXT => {
                match value.cast_string() {
                    Ok(ctx_name) => {
                        if self.autoadd {
                            self.stack.push_to_stack(name.clone(), value);
                        } else {
                            match self.to_stack(ctx_name) {
                                Ok(_) => {}
                                Err(err) => {
                                    bail!("Switching to a stack returns error: {}", err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        bail!("Can not get the name of context from the CONTEXT value: {}", err);
                    }
                }
            }
            _ => {
                if self.autoadd {
                    match self.stack.pull_from_stack(name.clone()) {
                        Some(mut val) => {
                            self.stack.push_to_stack(name.clone(), val.push(value));
                        }
                        None => {
                            bail!("Autoadd found no working data on stack");
                        }
                    }
                } else {
                    self.stack.push_to_stack(name.clone(), value);
                }
            }
        }
        Ok(self)
    }
}
