use crate::multistackvm::VM;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

pub fn stdlib_class_register(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline make");
    }
    match vm.stack.pull() {
        Some(class_value) => {
            if ! class_value.typoe_of() == CLASS {
                bail!("make expecting a lambda");
            }
            match vm.stack.pull() {
                Some(name_value) => {
                    match name_value.cast_string() {
                        Ok(name) => {
                            return vm.register_class(name, class_value);
                        }
                        Err(err) => {
                            bail!("MAKE expecting class name to be string: {}", err);
                        }
                    }
                }
                None => {
                    bail!("REGISTER returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("REGISTER returns: NO DATA #1");
        }
    }
}

pub fn stdlib_lambda_unregister(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline unregister");
    }

    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    return vm.unregister_lambda(name);
                }
                Err(err) => {
                    bail!("UNREGISTER expecting lanbda name to be string: {}", err);
                }
            }
        }
        None => {
            bail!("UNREGISTER returns: NO DATA #1");
        }
    }
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("register".to_string(), stdlib_lambda_register);
    let _ = vm.register_inline("unregister".to_string(), stdlib_lambda_unregister);
}
