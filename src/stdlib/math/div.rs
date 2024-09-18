use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::math::Ops;
use easy_error::{Error, bail};

pub fn stdlib_div_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline div");
    }
    match vm.stack.pull() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Div, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push(nvalue);
                        }
                        Err(err) => {
                            bail!("DIV returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("DIV returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("DIV returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_div_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline div_workbench");
    }
    match vm.stack.pull_from_workbench() {
        Some(value1) => {
            match vm.stack.pull() {
                Some(value2) => {
                    match Value::numeric_op(Ops::Div, value1, value2) {
                        Ok(nvalue) => {
                            vm.stack.push_to_workbench(nvalue);
                        }
                        Err(err) => {
                            bail!("DIVWORKBENCH returns error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("DIVWORKBENCH returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("DIVWORKBENCH returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("/".to_string(), stdlib_div_inline);
    let _ = vm.register_inline("/.".to_string(), stdlib_div_workbench_inline);
}
