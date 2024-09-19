use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use convert_case::{Case, Casing};
use easy_error::{Error, bail};

pub fn stdlib_string_case_upper(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline string_upper");
    }
    match vm.stack.pull() {
        Some(value) => {
            let data = match value.conv(STRING) {
                Ok(data) => data,
                Err(err) => {
                    bail!("STRING_UPPER return error: {}", err);
                }
            };
            match data.cast_string() {
                Ok(str_data) => {
                    vm.stack.push(Value::from_string(str_data.to_case(Case::Upper)));
                }
                Err(err) => {
                    bail!("STRING_UPPER return error: {}", err);
                }
            }
        }
        None => {
            bail!("STRING_UPPER returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_string_case_lower(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline string_lower");
    }
    match vm.stack.pull() {
        Some(value) => {
            let data = match value.conv(STRING) {
                Ok(data) => data,
                Err(err) => {
                    bail!("STRING_LOWER return error: {}", err);
                }
            };
            match data.cast_string() {
                Ok(str_data) => {
                    vm.stack.push(Value::from_string(str_data.to_case(Case::Lower)));
                }
                Err(err) => {
                    bail!("STRING_LOWER return error: {}", err);
                }
            }
        }
        None => {
            bail!("STRING_LOWER returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_string_case_snake(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline string_snake");
    }
    match vm.stack.pull() {
        Some(value) => {
            let data = match value.conv(STRING) {
                Ok(data) => data,
                Err(err) => {
                    bail!("STRING_LOWER return error: {}", err);
                }
            };
            match data.cast_string() {
                Ok(str_data) => {
                    vm.stack.push(Value::from_string(str_data.to_case(Case::Snake)));
                }
                Err(err) => {
                    bail!("STRING_SNAKE return error: {}", err);
                }
            }
        }
        None => {
            bail!("STRING_SNAKE returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_string_case_title(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline string_title");
    }
    match vm.stack.pull() {
        Some(value) => {
            let data = match value.conv(STRING) {
                Ok(data) => data,
                Err(err) => {
                    bail!("STRING_TITLE return error: {}", err);
                }
            };
            match data.cast_string() {
                Ok(str_data) => {
                    vm.stack.push(Value::from_string(str_data.to_case(Case::Title)));
                }
                Err(err) => {
                    bail!("STRING_TITLE return error: {}", err);
                }
            }
        }
        None => {
            bail!("STRING_TITLE returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_string_case_camel(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline string_camel");
    }
    match vm.stack.pull() {
        Some(value) => {
            let data = match value.conv(STRING) {
                Ok(data) => data,
                Err(err) => {
                    bail!("STRING_CAMEL return error: {}", err);
                }
            };
            match data.cast_string() {
                Ok(str_data) => {
                    vm.stack.push(Value::from_string(str_data.to_case(Case::Camel)));
                }
                Err(err) => {
                    bail!("STRING_CAMEL return error: {}", err);
                }
            }
        }
        None => {
            bail!("STRING_CAMEL returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("string.upper".to_string(), stdlib_string_case_upper);
    let _ = vm.register_inline("string.lower".to_string(), stdlib_string_case_lower);
    let _ = vm.register_inline("string.snake".to_string(), stdlib_string_case_snake);
    let _ = vm.register_inline("string.title".to_string(), stdlib_string_case_title);
    let _ = vm.register_inline("string.camel".to_string(), stdlib_string_case_camel);
}
