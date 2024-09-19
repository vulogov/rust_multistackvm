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

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("string.upper".to_string(), stdlib_string_case_upper);
}
