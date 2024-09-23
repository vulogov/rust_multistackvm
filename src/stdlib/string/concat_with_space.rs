use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

pub fn stdlib_string_concat_with_space(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline concat_with_space");
    }
    match vm.stack.pull() {
        Some(value) => {
            let data = match value.conv(STRING) {
                Ok(data) => data,
                Err(err) => {
                    bail!("CONCAT_WITH_SPACE return error: {}", err);
                }
            };
            match data.cast_string() {
                Ok(str_data) => {
                    match vm.stack.pull() {
                        Some(buffer) => {
                            if buffer.type_of() == TEXTBUFFER {
                                let nval = if buffer.len() == 0 {
                                    Value::from_string(format!("{}", str_data))
                                } else {
                                    Value::from_string(format!(" {}", str_data))
                                };
                                vm.stack.push(buffer + nval);
                            } else {
                                bail!("No textbuffer was found on stack");
                            }
                        }
                        None => {
                            bail!("CONCAT_WITH_SPACE returns: NO DATA #2");
                        }
                    }
                }
                Err(err) => {
                    bail!("CONCAT_WITH_SPACE return error: {}", err);
                }
            }
        }
        None => {
            bail!("CONCAT_WITH_SPACE returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("concat_with_space".to_string(), stdlib_string_concat_with_space);
}
