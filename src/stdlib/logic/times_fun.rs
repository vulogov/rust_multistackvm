use rust_dynamic::value::Value;
use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

pub fn stdlib_logic_times(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline times");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull() {
                    Some(n_val) => {
                        match n_val.cast_int() {
                            Ok(n) => {
                                for v in 0..n {
                                    vm.stack.push(Value::from_int(v));
                                    match vm.lambda_eval(lambda_val.clone()) {
                                        Ok(_) => continue,
                                        Err(err) => {
                                            bail!("TIMES: lambda execution returns error: {}", err);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                bail!("TIMES returns error: {}", err);
                            }
                        }
                    }
                    None => {
                        bail!("TIMES returns: NO DATA #2");
                    }
                }
            } else {
                bail!("TIMES: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("TIMES returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn stdlib_logic_times_from_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline times");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull_from_workbench() {
                    Some(n_val) => {
                        match n_val.cast_int() {
                            Ok(n) => {
                                for v in 0..n {
                                    vm.stack.push(Value::from_int(v));
                                    match vm.lambda_eval(lambda_val.clone()) {
                                        Ok(_) => continue,
                                        Err(err) => {
                                            bail!("TIMES: lambda execution returns error: {}", err);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                bail!("TIMES returns error: {}", err);
                            }
                        }
                    }
                    None => {
                        bail!("TIMES returns: NO DATA #2");
                    }
                }
            } else {
                bail!("TIMES: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("TIMES returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("times".to_string(), stdlib_logic_times);
    let _ = vm.register_inline("times.".to_string(), stdlib_logic_times_from_workbench);
}
