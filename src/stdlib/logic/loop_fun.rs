use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

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

pub fn stdlib_logic_loop(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_loop_base(vm, 2, StackOps::FromStack, "LOOP".to_string())
}

pub fn stdlib_logic_loop_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_loop_base(vm, 1, StackOps::FromWorkBench, "LOOP.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("loop".to_string(), stdlib_logic_loop);
    let _ = vm.register_inline("loop.".to_string(), stdlib_logic_loop_in_workbench);
}
