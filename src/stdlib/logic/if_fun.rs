use crate::multistackvm::{VM, StackOps};
use rust_dynamic::types::LAMBDA;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum TypeCond {
    IfTrue,
    IfFalse,
}

#[time_graph::instrument]
fn stdlib_logic_if_base(vm: &mut VM, cond: TypeCond, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 2 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    match vm.stack.pull() {
        Some(lambda_val) => {
            match lambda_val.type_of() {
                LAMBDA => {
                    let cond_v = match op {
                        StackOps::FromStack => vm.stack.pull(),
                        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
                    };
                    match cond_v {
                        Some(condition_val) => {
                            match condition_val.cast_bool() {
                                Ok(cond_bool) => {
                                    match cond {
                                        TypeCond::IfTrue => {
                                            if cond_bool {
                                                match lambda_val.type_of() {
                                                    LAMBDA => {
                                                        return vm.lambda_eval(lambda_val);
                                                    }
                                                    _ => {
                                                        vm.stack.push(lambda_val);
                                                        return vm.i("execute".to_string());
                                                    }
                                                }
                                            }
                                        }
                                        TypeCond::IfFalse => {
                                            if ! cond_bool {
                                                match lambda_val.type_of() {
                                                    LAMBDA => {
                                                        return vm.lambda_eval(lambda_val);
                                                    }
                                                    _ => {
                                                        vm.stack.push(lambda_val);
                                                        return vm.i("execute".to_string());
                                                    }
                                                }
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
                }
                _ => {
                    bail!("{}: #1 parameter must be lambda", &err_prefix);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_logic_if(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_if_base(vm, TypeCond::IfTrue, StackOps::FromStack, "IF".to_string())
}

pub fn stdlib_logic_if_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_if_base(vm, TypeCond::IfTrue, StackOps::FromWorkBench, "IF.".to_string())
}

pub fn stdlib_logic_if_false(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_if_base(vm, TypeCond::IfFalse, StackOps::FromStack, "?FALSE".to_string())
}

pub fn stdlib_logic_if_false_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_if_base(vm, TypeCond::IfFalse, StackOps::FromStack, "?FALSE.".to_string())
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
    let _ = vm.register_inline("if.in_workbench".to_string(), stdlib_logic_if_in_workbench);
    let _ = vm.register_inline("if.false".to_string(), stdlib_logic_if_false);
    let _ = vm.register_inline("if.false.in_workbench".to_string(), stdlib_logic_if_false_in_workbench);
    let _ = vm.register_inline("if.stack".to_string(), stdlib_logic_if_stack);
}
