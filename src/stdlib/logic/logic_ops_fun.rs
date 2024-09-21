use crate::multistackvm::VM;
use rust_dynamic::value::Value;
use rust_dynamic::types::BOOL;
use easy_error::{Error, bail};




pub fn stdlib_logic_op_not(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline not");
    }
    match vm.stack.pull() {
        Some(val) => {
            match val.conv(BOOL) {
                Ok(value) => {
                    match value.cast_bool() {
                        Ok(bool_val) => {
                            vm.stack.push(Value::from_bool(! bool_val));
                        }
                        Err(err) => {
                            bail!("NOT returns error: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("NOT returns error during boolean conversion: {}", err);
                }
            }
        }
        None => {
            bail!("NOT returns: NO DATA #1");
        }
    }
    Ok(vm)
}


pub fn stdlib_logic_op_and(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline and");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("AND returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("AND returns: NO DATA #2");
        }
    };
    let bool_val1 = match value1.conv(BOOL) {
        Ok(value) => {
            match value.cast_bool() {
                Ok(bool_val) => bool_val,
                Err(err) => {
                    bail!("AND returns error: {}", err);
                }
            }
        }
        Err(err) => {
            bail!("AND returns error during boolean conversion: {}", err);
        }
    };
    let bool_val2 = match value2.conv(BOOL) {
        Ok(value) => {
            match value.cast_bool() {
                Ok(bool_val) => bool_val,
                Err(err) => {
                    bail!("AND returns error: {}", err);
                }
            }
        }
        Err(err) => {
            bail!("AND returns error during boolean conversion: {}", err);
        }
    };
    vm.stack.push(Value::from_bool(bool_val1 & bool_val2));
    Ok(vm)
}

pub fn stdlib_logic_op_or(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline and");
    }
    let value1 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("AND returns: NO DATA #1");
        }
    };
    let value2 = match vm.stack.pull() {
        Some(val) => val,
        None => {
            bail!("AND returns: NO DATA #2");
        }
    };
    let bool_val1 = match value1.conv(BOOL) {
        Ok(value) => {
            match value.cast_bool() {
                Ok(bool_val) => bool_val,
                Err(err) => {
                    bail!("AND returns error: {}", err);
                }
            }
        }
        Err(err) => {
            bail!("AND returns error during boolean conversion: {}", err);
        }
    };
    let bool_val2 = match value2.conv(BOOL) {
        Ok(value) => {
            match value.cast_bool() {
                Ok(bool_val) => bool_val,
                Err(err) => {
                    bail!("AND returns error: {}", err);
                }
            }
        }
        Err(err) => {
            bail!("AND returns error during boolean conversion: {}", err);
        }
    };
    vm.stack.push(Value::from_bool(bool_val1 | bool_val2));
    Ok(vm)
}

pub fn init_stdlib(vm: &mut VM) {
    let _ = vm.register_inline("not".to_string(), stdlib_logic_op_not);
    let _ = vm.register_inline("and".to_string(), stdlib_logic_op_and);
    let _ = vm.register_inline("or".to_string(), stdlib_logic_op_or);
}
