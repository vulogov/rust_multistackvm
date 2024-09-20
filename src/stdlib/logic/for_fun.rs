use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};




pub fn stdlib_logic_for(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline for");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.lambda_eval(lambda_val.clone()) {
                        Ok(_) => {
                            match vm.stack.pull() {
                                Some(condition_val) => {
                                    match condition_val.cast_bool() {
                                        Ok(cond) => {
                                            if cond {
                                                continue;
                                            } else {
                                                break;
                                            }
                                        }
                                        Err(err) => {
                                            bail!("LOOP returns error: {}", err);
                                        }
                                    }
                                }
                                None => {
                                    bail!("LOOP returns: NO DATA #2");
                                }
                            }
                        }
                        Err(err) => {
                            bail!("FOR: lambda execution returns error: {}", err);
                        }
                    }
                }

            } else {
                bail!("FOR: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("FOR returns: NO DATA #1");
        }
    }
    Ok(vm)
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("for".to_string(), stdlib_logic_for);
}
