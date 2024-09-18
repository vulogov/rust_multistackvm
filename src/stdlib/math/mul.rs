use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::math::Ops;
use easy_error::{Error, bail};

pub fn stdlib_mul_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline mul");
    }
    match vm.stack.pull() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Mul, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push(nvalue);
                        }
                        Err(err) => {
                            bail!("MUL returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("MUL returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("MUL returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_mul_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline mul_workbench");
    }
    match vm.stack.pull_from_workbench() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Mul, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push_to_workbench(nvalue);
                        }
                        Err(err) => {
                            bail!("MULWORKBENCH returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("MULWORKBENCH returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("MULWORKBENCH returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("*".to_string(), stdlib_mul_inline);
    let _ = vm.register_inline("*.".to_string(), stdlib_mul_workbench_inline);
}
