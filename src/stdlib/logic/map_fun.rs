use crate::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

fn stdlib_logic_map_base(vm: &mut VM, depth: usize, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
                        let cv_name = &condition_val.type_name();
                        match condition_val.type_of() {
                            LIST => {
                                match condition_val.cast_list() {
                                    Ok(cond) => {
                                        let mut res = Value::list();
                                        for v in cond {
                                            vm.stack.push(v);
                                            match vm.lambda_eval(lambda_val.clone()) {
                                                Ok(_) => {
                                                    match vm.stack.pull() {
                                                        Some(outcome) => {
                                                            res = res.push(outcome);
                                                        }
                                                        None => {
                                                            bail!("{} can not obtain MAP outcome from stack", &err_prefix);
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    bail!("{}: lambda execution returns error: {}", &err_prefix, err);
                                                }
                                            }
                                        }
                                        match op {
                                            StackOps::FromStack => vm.stack.push(res),
                                            StackOps::FromWorkBench => vm.stack.push_to_workbench(res),
                                        };
                                    }
                                    Err(err) => {
                                        bail!("{} returns error: {}", &err_prefix, err);
                                    }
                                }
                            }
                            MATRIX => {
                                match condition_val.cast_matrix() {
                                    Ok(cond) => {
                                        let mut res: Vec<Vec<Value>> = Vec::new();
                                        for r in cond {
                                            let mut row: Vec<Value> = Vec::new();
                                            for v in r {
                                                vm.stack.push(v);
                                                match vm.lambda_eval(lambda_val.clone()) {
                                                    Ok(_) => {
                                                        match vm.stack.pull() {
                                                            Some(outcome) => {
                                                                 row.push(outcome);
                                                            }
                                                            None => {
                                                                bail!("{} can not obtain MAP outcome from stack", &err_prefix);
                                                            }
                                                        }
                                                    }
                                                    Err(err) => {
                                                        bail!("{}: lambda execution returns error: {}", &err_prefix, err);
                                                    }
                                                }
                                            }
                                            res.push(row);
                                        }
                                        match op {
                                            StackOps::FromStack => vm.stack.push(Value::from_matrix(res)),
                                            StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_matrix(res)),
                                        };
                                    }
                                    Err(err) => {
                                        bail!("{} returns error: {}", &err_prefix, err);
                                    }
                                }
                            }
                            _ => {
                                bail!("{}: can not run map over {}", &err_prefix, cv_name);
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

pub fn stdlib_logic_map(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_map_base(vm, 2, StackOps::FromStack, "MAP".to_string())
}

pub fn stdlib_logic_map_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_logic_map_base(vm, 1, StackOps::FromWorkBench, "MAP.".to_string())
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("map".to_string(), stdlib_logic_map);
    let _ = vm.register_inline("map.".to_string(), stdlib_logic_map_in_workbench);
}
