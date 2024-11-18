use crate::multistackvm::{VM};
use rust_dynamic::types::*;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum MoveOps {
    ToStack,
    ToWorkbench,
}

fn stdlib_stack_conditional_move_base(vm: &mut VM, op: MoveOps, err_prefix: String) -> Result<&mut VM, Error> {

    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline {}()", &err_prefix);
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            match lambda_val.type_of() {
                LAMBDA => {
                    match vm.lambda_eval(lambda_val) {
                        Ok(_) => {}
                        Err(err) => {
                            bail!("{} lambda returns: {}", &err_prefix, err);
                        }
                    }
                    let outcome = match vm.stack.pull() {
                        Some(outcome) => outcome,
                        None => {
                            bail!("{} returns: NO OUTCOME", &err_prefix);
                        }
                    };
                    let outcome_bool = match outcome.cast_bool() {
                        Ok(outcome_bool) => outcome_bool,
                        Err(err) => {
                            bail!("{} returns: OUTCOME IS NOT BOOLEAN: {}", &err_prefix, err);
                        }
                    };
                    let value = match vm.stack.pull() {
                        Some(value) => value,
                        None => {
                            bail!("{} returns: NO VALUE TO MOVE", &err_prefix);
                        }
                    };
                    if outcome_bool {
                        match op {
                            MoveOps::ToStack => {
                                match vm.stack.pull() {
                                    Some(name_val) => {
                                        match name_val.cast_string() {
                                            Ok(stack_name) => {
                                                vm.stack.push_to_stack(stack_name, value);
                                            }
                                            Err(err) => {
                                                bail!("{} returns: {}", &err_prefix, err);
                                            }
                                        }
                                    }
                                    None => {
                                        bail!("{} returns: NO DATA for stack name", &err_prefix);
                                    }
                                }
                            }
                            MoveOps::ToWorkbench => {
                                vm.stack.push_to_workbench(value);
                            }
                        }
                    }
                }
                _ => {
                    bail!("{} expected to have a LAMBDA at #1", &err_prefix);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_stack_conditional_move_to_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_stack_conditional_move_base(vm, MoveOps::ToStack, "?MOVE".to_string())
}

pub fn stdlib_stack_conditional_move_to_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_stack_conditional_move_base(vm, MoveOps::ToWorkbench, "?.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("?move".to_string(), stdlib_stack_conditional_move_to_stack);
    let _ = vm.register_inline("?.".to_string(), stdlib_stack_conditional_move_to_workbench);
}
