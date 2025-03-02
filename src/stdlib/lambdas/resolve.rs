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
                    let real_name = match vm.get_alias(name.clone()) {
                        Ok(real_name) => real_name,
                        Err(_) => name.clone(),
                    };
                    if vm.is_lambda(real_name.clone()) {
                        return vm.apply(Value::ptr(real_name.clone(), Vec::new()));
                    } else if vm.is_inline(real_name.clone()) {
                        return vm.apply(Value::ptr(real_name.clone(), Vec::new()));
                    } else if vm.stack.is_inline(real_name.clone()) {
                        return vm.apply(Value::ptr(real_name.clone(), Vec::new()));
                    } else {
                        bail!("RESOLVE: function {} not found", &name);
                    }
                }
                Err(err) => {
                    bail!("RESOLVE returns error: {}", err);
                }
            }
        }
        None => {
            bail!("RESOLVE returns: NO DATA");
        }
    }
}

pub fn stdlib_class_resolve(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline resolve.class()");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            match name_value.cast_string() {
                Ok(name) => {
                    if vm.is_class(name.clone()) {
                        let c = match vm.get_class(name.clone()) {
                            Ok(c) => c,
                            Err(err) => bail!("RESOLVE.CLASS failed to obtain {}: {}", &name, err),
                        };
                        vm.stack.push(c);
                    } else {
                        bail!("RESOLVE.CLASS class {} not registered", &name);
                    }
                }
                Err(err) => {
                    bail!("RESOLVE.CLASS returns error: {}", err);
                }
            }
        }
        None => {
            bail!("RESOLVE.CLASS returns: NO DATA");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("resolve".to_string(), stdlib_function_resolve);
    let _ = vm.register_inline("resolve.class".to_string(), stdlib_class_resolve);
}
