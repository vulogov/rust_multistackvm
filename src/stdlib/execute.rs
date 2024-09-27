use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::*;
use easy_error::{Error, bail};

pub fn stdlib_execute_base_inline(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline {}()", &err_prefix);
    }
    let recv_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match recv_value {
        Some(mut ptr_value) => {
            match ptr_value.type_of() {
                PTR | STRING => {
                    match &mut ptr_value.data {
                        Val::String(ref mut fun_name) => {
                            return vm.call(fun_name.clone());
                        }
                        _ => {
                            bail!("{} not returned a proper function name", &err_prefix);
                        }
                    }
                }
                LIST => {
                    match ptr_value.cast_list() {
                        Ok(list_val) => {
                            for v in list_val {
                                vm.stack.push(v);
                                match stdlib_execute_base_inline(vm, op.clone(), err_prefix.clone()) {
                                    Ok(_) => continue,
                                    Err(err) => {
                                        bail!("{}", err);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            bail!("{} returned error during unfolding the list: {}", &err_prefix, err);
                        }
                    }
                }
                LAMBDA => {
                    return vm.lambda_eval(ptr_value);
                }
                _ => {
                    bail!("Received value is not of executable type");
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA", err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_execute_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline execute()");
    }
    stdlib_execute_base_inline(vm, StackOps::FromStack, "EXECUTE".to_string())
}

pub fn stdlib_execute_from_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline execute()");
    }
    stdlib_execute_base_inline(vm, StackOps::FromWorkBench, "EXECUTE.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("execute".to_string(), stdlib_execute_inline);
    let _ = vm.register_inline("execute.".to_string(), stdlib_execute_from_workbench_inline);
}
