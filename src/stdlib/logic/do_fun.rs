use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

pub fn stdlib_logic_do(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline do");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.lambda_eval(lambda_val.clone()) {
                        Ok(_) => {
                            if vm.stack.current_stack_len() == 0 {
                                break;
                            }
                        }
                        Err(err) => {
                            bail!("DO: lambda execution returns error: {}", err);
                        }
                    }
                }
            } else {
                bail!("DO: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("DO returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_do_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.workbench.len() < 1 {
        bail!("Workbench is too shallow for inline do");
    }
    match vm.stack.pull_from_workbench() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.lambda_eval(lambda_val.clone()) {
                        Ok(_) => {
                            if vm.stack.current_stack_len() == 0 {
                                break;
                            }
                        }
                        Err(err) => {
                            bail!("DO: lambda execution returns error: {}", err);
                        }
                    }
                }
            } else {
                bail!("DO: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("DO returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("do".to_string(), stdlib_logic_do);
    let _ = vm.register_inline("do.".to_string(), stdlib_logic_do_in_workbench);
}
