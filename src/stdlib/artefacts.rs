use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_list_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::list())
}

pub fn stdlib_lambda_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.apply(Value::lambda())
}

pub fn stdlib_ptr_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline ptr()");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    return vm.apply(Value::ptr(name.clone(), Vec::new()));
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
    let _ = vm.register_inline("list".to_string(), stdlib_list_inline);
    let _ = vm.register_inline("lambda".to_string(), stdlib_lambda_inline);
    let _ = vm.register_inline("ptr".to_string(), stdlib_ptr_inline);
}
