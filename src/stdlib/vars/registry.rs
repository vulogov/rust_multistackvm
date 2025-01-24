use crate::multistackvm::VM;
use easy_error::{Error, bail};

pub fn stdlib_var_register(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline var");
    }
    match vm.stack.pull() {
        Some(var_value) => {
            match vm.stack.pull() {
                Some(name_value) => {
                    match name_value.cast_string() {
                        Ok(name) => {
                            return vm.register_var(name, var_value);
                        }
                        Err(err) => {
                            bail!("VAR expecting var name to be string: {}", err);
                        }
                    }
                }
                None => {
                    bail!("VAR returns: NO DATA #2");
                }
            }
        }
        None => {
            bail!("VAR returns: NO DATA #1");
        }
    }
}

pub fn stdlib_var_unregister(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline var-");
    }

    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    return vm.unregister_var(name);
                }
                Err(err) => {
                    bail!("VAR- expecting lanbda name to be string: {}", err);
                }
            }
        }
        None => {
            bail!("VAR- returns: NO DATA #1");
        }
    }
}



pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("var".to_string(), stdlib_var_register);
    let _ = vm.register_inline("var-".to_string(), stdlib_var_unregister);
}
