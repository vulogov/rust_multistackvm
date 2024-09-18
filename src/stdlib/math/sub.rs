use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::math::Ops;
use easy_error::{Error, bail};

pub fn stdlib_sub_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline sub");
    }
    match vm.stack.pull() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Sub, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push(nvalue);
                        }
                        Err(err) => {
                            bail!("SUB returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("SUB returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("SUB returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_sub_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline sub_workbench");
    }
    match vm.stack.pull_from_workbench() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Sub, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push_to_workbench(nvalue);
                        }
                        Err(err) => {
                            bail!("SUBWORKBENCH returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("SUBWORKBENCH returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("SUBWORKBENCH returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("-".to_string(), stdlib_sub_inline);
    let _ = vm.register_inline("-.".to_string(), stdlib_sub_workbench_inline);
}
