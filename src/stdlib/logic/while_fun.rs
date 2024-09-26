use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

pub fn stdlib_logic_while(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline while()");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.stack.pull() {
                        Some(condition_val) => {
                            match condition_val.cast_bool() {
                                Ok(cond) => {
                                    if cond {
                                        match vm.lambda_eval(lambda_val.clone()) {
                                            Ok(_) => continue,
                                            Err(err) => {
                                                bail!("WHILE: lambda execution returns error: {}", err);
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    bail!("WHILE returns error: {}", err);
                                }
                            }
                        }
                        None => {
                            bail!("WHILE returns: NO DATA #2");
                        }
                    }
                }
            } else {
                bail!("WHILE: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("WHILE returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn stdlib_logic_while_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline while.()");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.stack.pull_from_workbench() {
                        Some(condition_val) => {
                            match condition_val.cast_bool() {
                                Ok(cond) => {
                                    if cond {
                                        match vm.lambda_eval(lambda_val.clone()) {
                                            Ok(_) => continue,
                                            Err(err) => {
                                                bail!("WHILE.: lambda execution returns error: {}", err);
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    bail!("WHILE. returns error: {}", err);
                                }
                            }
                        }
                        None => {
                            bail!("WHILE. returns: NO DATA #2");
                        }
                    }
                }
            } else {
                bail!("WHILE.: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("WHILE. returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("while".to_string(), stdlib_logic_while);
    let _ = vm.register_inline("while.".to_string(), stdlib_logic_while_in_workbench);
}
