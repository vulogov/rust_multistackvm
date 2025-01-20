use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::*;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn stdlib_logic_loop_base(vm: &mut VM, depth: usize, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < depth {
        bail!("Stack is too shallow for inline {}", &err_prefix);
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                let cond = match op {
                    StackOps::FromStack => vm.stack.pull(),
                    StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
                };
                match cond {
                    Some(condition_val) => {
                        match condition_val.cast_list() {
                            Ok(cond) => {
                                for v in cond {
                                    vm.stack.push(v);
                                    match vm.lambda_eval(lambda_val.clone()) {
                                        Ok(_) => continue,
                                        Err(err) => {
                                            bail!("{}: lambda execution returns error: {}", &err_prefix, err);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                bail!("{} returns error: {}", &err_prefix, err);
                            }
                        }
                    }
                    None => {
                        bail!("{} returns: NO DATA #2", &err_prefix);
                    }
                }
            } else {
                bail!("{}: #1 parameter must be lambda", &err_prefix);
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

fn stdlib_logic_loop_over_stack_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let lambda_res = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match lambda_res {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                loop {
                    match vm.stack.peek() {
                        Some(next_value) => {
                            if next_value.type_of() == NODATA {
                                let _ = vm.stack.pull();
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                    if vm.stack.current_stack_len() > 0 {
                        match vm.lambda_eval(lambda_val.clone()) {
                            Ok(_) => continue,
                            Err(err) => {
                                bail!("{}: lambda execution returns error: {}", &err_prefix, err);
                            }
                        }
                    }
                }
            } else {
                bail!("{}: #1 parameter must be lambda", &err_prefix);
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_loop(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_loop_base(vm, 2, StackOps::FromStack, "LOOP".to_string())
}

pub fn stdlib_logic_loop_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_loop_base(vm, 1, StackOps::FromWorkBench, "LOOP.".to_string())
}

pub fn stdlib_logic_loop_over_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_loop_over_stack_base(vm, StackOps::FromStack, "*LOOP".to_string())
}

pub fn stdlib_logic_loop_over_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_loop_over_stack_base(vm, StackOps::FromStack, "*LOOP.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("loop".to_string(), stdlib_logic_loop);
    let _ = vm.register_inline("loop.".to_string(), stdlib_logic_loop_in_workbench);
    let _ = vm.register_inline("*loop".to_string(), stdlib_logic_loop_over_stack);
    let _ = vm.register_inline("*loop.".to_string(), stdlib_logic_loop_over_workbench);
}
