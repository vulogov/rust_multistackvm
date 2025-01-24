use crate::multistackvm::VM;
use easy_error::{Error, bail};

pub fn stdlib_function_resolve_var(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline VAR?");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    match vm.get_var(name) {
                        Ok(value) => {
                            vm.stack.push(value);
                        }
                        Err(err) => {
                            bail!("VAR? returned: {}", err);
                        }
                    };
                }
                Err(err) => {
                    bail!("VAR? returns error: {}", err);
                }
            }
        }
        None => {
            bail!("VAR? returns: NO DATA");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("var?".to_string(), stdlib_function_resolve_var);
}
