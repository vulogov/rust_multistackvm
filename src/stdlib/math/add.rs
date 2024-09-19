use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use rust_dynamic::math::Ops;
use easy_error::{Error, bail};

pub fn stdlib_add_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline add");
    }
    match vm.stack.pull() {
        Some(mut value1) => {
            println!("{:?}", &value1);
            match vm.stack.pull() {
                Some(value2) => {
                    match value1.dt {
                        LIST => {
                            match value2.dt {
                                LIST => {
                                    let mut res = Value::list();
                                    for v in value1 {
                                        res = res.push(v);
                                    }
                                    for v in value2 {
                                        res = res.push(v);
                                    }
                                    vm.stack.push(res);
                                }
                                _ => {
                                    vm.stack.push(value1.push(value2));
                                }
                            }
                        }
                        _ => {
                            match Value::numeric_op(Ops::Add, value1, value2) {
                                Ok(nvalue) => {
                                    vm.stack.push(nvalue);
                                }
                                Err(err) => {
                                    bail!("ADD returns error: {}", err);
                                }
                            }
                        }
                    }
                }
                None => {
                    bail!("ADD returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("ADD returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_add_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline add_workbench");
    }
    match vm.stack.pull_from_workbench() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Add, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push_to_workbench(nvalue);
                        }
                        Err(err) => {
                            bail!("ADDWORKBENCH returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("ADDWORKBENCH returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("ADDWORKBENCH returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("+".to_string(), stdlib_add_inline);
    let _ = vm.register_inline("+.".to_string(), stdlib_add_workbench_inline);
}
