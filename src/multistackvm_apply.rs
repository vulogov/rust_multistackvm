use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::multistackvm::*;
use easy_error::{bail, Error};

impl VM {
    #[time_graph::instrument]
    pub fn apply(&mut self, value: Value) -> Result<&mut VM, Error> {
        match value.dt {
            CALL => {
                match value.cast_string() {
                    Ok(fun_name) => {
                        if fun_name.len() == 0 {
                            bail!("Empty function name passed for CALL");
                        }
                        if self.is_command(fun_name.clone()) {
                            return self.c(fun_name.clone());
                        } else {
                            if self.autoadd {
                                match self.stack.pull() {
                                    Some(mut val) => {
                                        self.stack.push(val.push(value));
                                    }
                                    None => {
                                        bail!("Autoadd found no working data on stack");
                                    }
                                }
                            } else {
                                //
                                // If function name starts with '$' we are forcing to call internal function
                                // without lambda check or alias resolution
                                //
                                if fun_name.chars().nth(0).unwrap() == '$' {
                                    return self.call_internal_word(fun_name.clone());
                                }
                                //
                                // Or we start with alias resolution
                                //
                                let real_name = match self.get_alias(fun_name.clone()) {
                                    Ok(real_name) => real_name,
                                    Err(_) => fun_name.clone(),
                                };
                                //
                                // If function is lambda
                                //
                                if self.is_lambda(real_name.clone()) {
                                    match self.get_lambda(real_name.clone()) {
                                        Ok(lambda) => {
                                            return self.lambda_eval(lambda);
                                        }
                                        Err(err) => {
                                            bail!("Error getting lambda {}: {}", &fun_name, err);
                                        }
                                    }
                                } else {
                                    //
                                    // Otherwise call an inline
                                    //
                                    return self.i(real_name.clone());
                                }
                            }
                        }
                    }
                    Err(err) => {
                        bail!("Can not get the name of function from the CALL value: {}", err);
                    }
                }
            }
            CONTEXT => {
                match value.cast_string() {
                    Ok(ctx_name) => {
                        if self.autoadd {
                            self.stack.push(value);
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
                    match self.stack.pull() {
                        Some(mut val) => {
                            self.stack.push(val.push(value));
                        }
                        None => {
                            bail!("Autoadd found no working data on stack");
                        }
                    }
                } else {
                    self.stack.push(value);
                }
            }
        }
        Ok(self)
    }
}
