use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_function_resolve(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline resolve()");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    if vm.is_lambda(name.clone()) {
                        return vm.apply(Value::ptr(name.clone(), Vec::new()));
                    } else if vm.is_inline(name.clone()) {
                        return vm.apply(Value::ptr(name.clone(), Vec::new()));
                    } else if vm.stack.is_inline(name.clone()) {
                        return vm.apply(Value::ptr(name.clone(), Vec::new()));
                    } else {
                        bail!("RESOLVE: function {} not found", &name);
                    }
                }
                Err(err) => {
                    bail!("PTR returns error: {}", err);
                }
            }
        }
        None => {
            bail!("PTR returns: NO DATA");
        }
    }
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("resolve".to_string(), stdlib_function_resolve);
}
