use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use serde_json::{Value as JSON_Value, from_str as json_from_str};
use easy_error::{Error, bail};


pub fn stdlib_json_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline json()");
    }
    match vm.stack.pull() {
        Some(data_value) => {
            match data_value.cast_string() {
                Ok(data_json) => {
                    match json_from_str::<JSON_Value>(&data_json) {
                        Ok(j_value) => {
                            return vm.apply(Value::json(j_value.clone()));
                        }
                        Err(err) => {
                            bail!("JSON convert returns: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("JSON returns error: {}", err);
                }
            }
        }
        None => {
            bail!("JSON returns: NO DATA");
        }
    }
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("json".to_string(), stdlib_json_inline);
}
