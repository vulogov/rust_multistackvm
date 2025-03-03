use crate::multistackvm::{VM, StackOps};
use crate::stdlib;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_execute_base_inline(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Stack is too shallow for inline {}()", &err_prefix);
            }
        }
    }
    let recv_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match recv_value {
        Some(mut ptr_value) => {
            match ptr_value.type_of() {
                PTR | STRING | CALL => {
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
                MAP | INFO | CONFIG | ASSOCIATION => {
                    let key_val = match vm.stack.pull() {
                        Some(key_val) => key_val,
                        None => {
                            bail!("{} can not obtain key for DICT execute", &err_prefix);
                        }
                    };
                    match key_val.cast_string() {
                        Ok(key) => {
                            match ptr_value.get(key) {
                                Ok(exec_val) => {
                                    vm.stack.push(exec_val);
                                    match stdlib_execute_base_inline(vm, op.clone(), err_prefix.clone()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            bail!("{}", err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("{} returned error during DICT execute: {}", &err_prefix, err);
                                }
                            }
                        }
                        Err(err) => {
                            bail!("{} returned error during DICT key conversion: {}", &err_prefix, err);
                        }
                    }
                }
                CONDITIONAL => {
                    return stdlib::execute_types::execute_conditionals::execute_conditionals(vm, ptr_value, op.clone(), err_prefix.clone());
                }
                CLASS => {
                    return stdlib::bund_execute::execute_class::execute_class(vm, ptr_value, op.clone(), err_prefix.clone());
                }
                OBJECT => {
                    return stdlib::bund_execute::execute_object::execute_object(vm, ptr_value, op.clone(), err_prefix.clone());
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

#[time_graph::instrument]
pub fn stdlib_execute_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline execute()");
    }
    stdlib_execute_base_inline(vm, StackOps::FromStack, "EXECUTE".to_string())
}
#[time_graph::instrument]
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
