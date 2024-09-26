use crate::multistackvm::VM;
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

pub fn stdlib_logic_loop(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline loop");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull() {
                    Some(condition_val) => {
                        match condition_val.cast_list() {
                            Ok(cond) => {
                                for v in cond {
                                    vm.stack.push(v);
                                    match vm.lambda_eval(lambda_val.clone()) {
                                        Ok(_) => continue,
                                        Err(err) => {
                                            bail!("LOOP: lambda execution returns error: {}", err);
                                        }
                                    }
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
            } else {
                bail!("LOOP: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("LOOP returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_loop_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline loop.workbench()");
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            if lambda_val.is_type(LAMBDA) {
                match vm.stack.pull_from_workbench() {
                    Some(condition_val) => {
                        match condition_val.cast_list() {
                            Ok(cond) => {
                                for v in cond {
                                    vm.stack.push(v);
                                    match vm.lambda_eval(lambda_val.clone()) {
                                        Ok(_) => continue,
                                        Err(err) => {
                                            bail!("LOOP.: lambda execution returns error: {}", err);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                bail!("LOOP. returns error: {}", err);
                            }
                        }
                    }
                    None => {
                        bail!("LOOP. returns: NO DATA #2");
                    }
                }
            } else {
                bail!("LOOP.: #1 parameter must be lambda");
            }
        }
        None => {
            bail!("LOOP. returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("loop".to_string(), stdlib_logic_loop);
    let _ = vm.register_inline("loop.".to_string(), stdlib_logic_loop_in_workbench);
}
