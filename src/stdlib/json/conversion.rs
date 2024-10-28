use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::JSON;
use easy_error::{Error, bail};

pub fn stdlib_json_from_value(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline json.from_value");
    }
    match vm.stack.pull() {
        Some(value) => {
            if value.is_type(JSON) {
                bail!("Stack already having a JSON value");
            }
            match value.cast_value_to_json() {
                Ok(j_value) => {
                    vm.stack.push(Value::json(j_value));
                }
                Err(err) => {
                    bail!("Error casting JSON value: {}", err);
                }
            }
        }
        None => {
            bail!("JSON.FROM_VALUE returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_json_to_value(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline json.to_value");
    }
    match vm.stack.pull() {
        Some(j_value) => {
            if j_value.is_type(JSON) {
                match j_value.cast_json_to_value() {
                    Ok(value) => {
                        vm.stack.push(value);
                    }
                    Err(err) => {
                        bail!("Error casting from JSON value: {}", err);
                    }
                }
            } else {
                bail!("JSON.TO_VALUE: #1 parameter must be JSON");
            }
        }
        None => {
            bail!("JSON.TO_VALUE returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("json.from_value".to_string(), stdlib_json_from_value);
    let _ = vm.register_inline("json.to_value".to_string(), stdlib_json_to_value);
}
