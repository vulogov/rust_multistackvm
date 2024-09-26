use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

pub fn stdlib_logic_if(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline if");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull() {
                    Some(condition_val) => {
                        match condition_val.cast_bool() {
                            Ok(cond) => {
                                if cond {
                                    return vm.lambda_eval(lambda_val);
                                }
                            }
                            Err(err) => {
                                bail!("IF returns error: {}", err);
                            }
                        }
                    }
                    None => {
                        bail!("IF returns: NO DATA #2");
                    }
                }
            } else {
                bail!("IF: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("IF returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_if_false(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline if");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull() {
                    Some(condition_val) => {
                        match condition_val.cast_bool() {
                            Ok(cond) => {
                                if ! cond {
                                    return vm.lambda_eval(lambda_val);
                                }
                            }
                            Err(err) => {
                                bail!("IF returns error: {}", err);
                            }
                        }
                    }
                    None => {
                        bail!("IF returns: NO DATA #2");
                    }
                }
            } else {
                bail!("IF: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("IF returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_if_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline if.stack");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull() {
                    Some(condition_val) => {
                        match condition_val.cast_string() {
                            Ok(cond) => {
                                if vm.stack.current_stack_name() == Some(cond) {
                                    return vm.lambda_eval(lambda_val);
                                }
                            }
                            Err(err) => {
                                bail!("IF.stack returns error: {}", err);
                            }
                        }
                    }
                    None => {
                        bail!("IF.stack returns: NO DATA #2");
                    }
                }
            } else {
                bail!("IF.stack: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("IF.stack returns: NO DATA #1");
        }
    }
    Ok(vm)
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("if".to_string(), stdlib_logic_if);
    let _ = vm.register_inline("if.false".to_string(), stdlib_logic_if_false);
    let _ = vm.register_inline("if.stack".to_string(), stdlib_logic_if_stack);
}
