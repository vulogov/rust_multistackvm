use crate::multistackvm::VM;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

pub fn stdlib_convert_to_string(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline convert_string");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.conv(STRING) {
                Ok(nvalue) => {
                    vm.stack.push(nvalue);
                }
                Err(err) => {
                    bail!("CONVERT.TO_STRING returned error: {}", err);
                }
            }
        }
        None => {
            bail!("CONVERT.TO_STRING returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_convert_to_int(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline convert_int");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.conv(INTEGER) {
                Ok(nvalue) => {
                    vm.stack.push(nvalue);
                }
                Err(err) => {
                    bail!("CONVERT.TO_INT returned error: {}", err);
                }
            }
        }
        None => {
            bail!("CONVERT.TO_INT returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_convert_to_float(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline convert_float");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.conv(FLOAT) {
                Ok(nvalue) => {
                    vm.stack.push(nvalue);
                }
                Err(err) => {
                    bail!("CONVERT.TO_FLOAT returned error: {}", err);
                }
            }
        }
        None => {
            bail!("CONVERT.TO_FLOAT returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_convert_to_bool(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline convert_bool");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.conv(BOOL) {
                Ok(nvalue) => {
                    vm.stack.push(nvalue);
                }
                Err(err) => {
                    bail!("CONVERT.TO_BOOL returned error: {}", err);
                }
            }
        }
        None => {
            bail!("CONVERT.TO_BOOL returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn stdlib_convert_to_list(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline convert_list");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.conv(LIST) {
                Ok(nvalue) => {
                    vm.stack.push(nvalue);
                }
                Err(err) => {
                    bail!("CONVERT.TO_LIST returned error: {}", err);
                }
            }
        }
        None => {
            bail!("CONVERT.TO_LIST returns: NO DATA #1");
        }
    }
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("convert.to_string".to_string(), stdlib_convert_to_string);
    let _ = vm.register_inline("convert.to_int".to_string(), stdlib_convert_to_int);
    let _ = vm.register_inline("convert.to_float".to_string(), stdlib_convert_to_float);
    let _ = vm.register_inline("convert.to_bool".to_string(), stdlib_convert_to_bool);
    let _ = vm.register_inline("convert.to_list".to_string(), stdlib_convert_to_list);
}
