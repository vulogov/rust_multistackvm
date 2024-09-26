use crate::multistackvm::VM;
use rust_dynamic::types::*;
use easy_error::{Error, bail};


pub fn stdlib_execute_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline execute()");
    }
    match vm.stack.pull() {
        Some(mut ptr_value) => {
            match ptr_value.dt {
                PTR | STRING => {
                    match &mut ptr_value.data {
                        Val::String(ref mut fun_name) => {
                            return vm.call(fun_name.clone());
                        }
                        _ => {
                            bail!("EXECUTE not returned a proper function name");
                        }
                    }
                }
                LAMBDA => {
                    return vm.lambda_eval(ptr_value);
                }
                _ => {
                    bail!("Value on the stack is not PTR type");
                }
            }
        }
        None => {
            bail!("EXECUTE returns: NO DATA");
        }
    }
}

pub fn stdlib_execute_from_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline execute()");
    }
    match vm.stack.pull_from_workbench() {
        Some(mut ptr_value) => {
            match ptr_value.dt {
                PTR | STRING => {
                    match &mut ptr_value.data {
                        Val::String(ref mut fun_name) => {
                            return vm.call(fun_name.clone());
                        }
                        _ => {
                            bail!("EXECUTE. not returned a proper function name");
                        }
                    }
                }
                LAMBDA => {
                    return vm.lambda_eval(ptr_value);
                }
                _ => {
                    bail!("Value on the stack is not executable type");
                }
            }
        }
        None => {
            bail!("EXECUTE. returns: NO DATA");
        }
    }
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("execute".to_string(), stdlib_execute_inline);
    let _ = vm.register_inline("execute.".to_string(), stdlib_execute_from_workbench_inline);
}
